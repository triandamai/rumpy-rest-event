use std::str::FromStr;

use crate::{
    common::{
        api_response::ApiResponse,
        app_state::AppState,
        constant::{
            COLLECTION_USERS, PROVIDER_OTP, REDIS_KEY_OTP, REDIS_KEY_OTP_AT, REDIS_KEY_OTP_ATTEMPT,
            REDIS_KEY_OTP_PHONE_NUMBER, REDIS_KEY_OTP_TYPE, REDIS_KEY_USER_ID,
            REDIS_KEY_USER_PHONE_NUMBER, REDIS_KEY_USER_TOKEN, REDIS_SESSION_OTP_SIGN_IN,
            REDIS_SESSION_OTP_SIGN_UP, REDIS_SESSION_SIGN_IN, USER_STATUS_ACTIVE,
            USER_STATUS_WAITING_ACTIVATION,
        },
        i18n,
        jwt::{JwtClaims, JwtUtil},
        lang::Lang,
        middleware::Json,
        mongo::{DB, filter::is},
        utils::{generate_otp, get_i64_with_default, get_naive_date_time, get_string_with_default},
        wa,
    },
    dto::user_dto::UserDTO,
    entity::{user::User, user_metadata::UserMetaData},
    i18n,
};
use auth_model::{AuthOTPRequest, AuthOTPResponse, VerifyOTPRequest, VerifyOTPResponse};
use axum::extract::State;
use bson::{DateTime, doc, oid::ObjectId};
use chrono::Local;
use log::info;
use validator::Validate;
pub mod auth_model;

pub async fn auth_otp(
    state: State<AppState>,
    lang: Lang,
    Json(mut body): Json<AuthOTPRequest>,
) -> ApiResponse<AuthOTPResponse> {
    info!(target:"auth_otp","starting...");
    let i18n = i18n!("auth", lang);

    let validate = body.validate();
    if let Err(err) = validate {
        info!(target:"auth_otp","{:?}",err);
        return ApiResponse::error_validation(err, &i18n.translate("auth_otp.validation_error"));
    }

    if body.phone_number.starts_with("08") {
        body.phone_number = body.phone_number.replace("08", "628");
    }
    if body.phone_number.contains("-") {
        body.phone_number = body.phone_number.replace("-", "");
    }
    if body.phone_number.contains("+") {
        body.phone_number = body.phone_number.replace("+", "");
    }
    if body.phone_number.contains(" ") {
        body.phone_number = body.phone_number.replace(" ", "");
    }

    let find_duplicate_phone_number = DB::get(COLLECTION_USERS)
        .filter(vec![is("phone_number", body.phone_number.clone())])
        .get_one::<UserDTO>(&state.db)
        .await;

    if let Ok(user) = find_duplicate_phone_number {
        info!(target:"auth_otp","phone number already exist signing...");
        sign_in_otp(state, i18n, user, body).await
    } else {
        info!(target:"auth_otp","account doesn't exist registering new account...");
        sign_up_otp(state, i18n, body).await
    }
}

pub async fn sign_up_otp(
    mut state: State<AppState>,
    i18n: i18n::I18n,
    body: AuthOTPRequest,
) -> ApiResponse<AuthOTPResponse> {
    let create_new_user_id = ObjectId::new();
    let user = User {
        id: Some(create_new_user_id),
        display_name: String::new(),
        email: String::new(),
        phone_number: body.phone_number.clone(),
        password: None,
        app_meta_data: None,
        user_meta_data: Some(UserMetaData {
            providers: Some(vec![PROVIDER_OTP.to_string()]),
        }),
        profile_picture: None,
        last_logged_in: None,
        status: Some(USER_STATUS_WAITING_ACTIVATION.to_string()),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        confirmation_at: None,
        confirmation_sent_at: None,
    };

    let session = state.db.start_session().await;
    if let Err(err) = session {
        info!(target:"sign_up_otp","{:?}",err);
        return ApiResponse::un_authorized(&i18n.translate("sign_up_otp.session_invalid"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let save_user = DB::insert(COLLECTION_USERS)
        .one_with_session(user, &state.db, &mut session)
        .await;

    if let Err(why) = save_user {
        info!(target:"sign_up_otp","{:?}",why);
        let _ = session.abort_transaction().await;
        return ApiResponse::un_authorized(&i18n.translate("sign_up_otp.failed_create_user"));
    }
    let session_id = ObjectId::new().to_string();

    let create_session_token =
        JwtUtil::encode(session_id.clone(), REDIS_SESSION_OTP_SIGN_UP.to_string());
    if let None = create_session_token {
        info!(target:"sign_up_otp","failed to create session token");
        let _ = session.abort_transaction().await;
        return ApiResponse::un_authorized(&i18n.translate("sign_in_otp.failed_create_token"));
    }

    let create_otp = generate_otp();
    let created_at = chrono::Local::now().naive_local().and_utc().timestamp();

    let send_otp = wa::send_otp(body.phone_number.clone(), create_otp.clone()).await;

    if let Err(why) = send_otp {
        info!(target:"sign_up_otp","failed to send otp {:?}",why);
        let _ = session.abort_transaction().await;
        return ApiResponse::un_authorized(&i18n.translate("sign_in_otp.otp_not_sent"));
    }

    let create_otp_session = &state.redis.set_session_otp(
        &session_id,
        &[
            (REDIS_KEY_USER_ID, create_new_user_id.to_string()),
            (REDIS_KEY_USER_TOKEN, create_session_token.clone().unwrap()),
            (REDIS_KEY_OTP_PHONE_NUMBER, body.phone_number.clone()),
            (REDIS_KEY_OTP, create_otp),
            (REDIS_KEY_OTP_TYPE, REDIS_SESSION_OTP_SIGN_UP.to_string()),
            (REDIS_KEY_OTP_AT, created_at.to_string()),
            (REDIS_KEY_OTP_ATTEMPT, "0".to_string()),
        ],
    );

    if let Err(why) = create_otp_session {
        info!(target:"sign_up_otp","failed to save session {:?}",why);
        let _ = session.abort_transaction().await;
        return ApiResponse::un_authorized(&i18n.translate("sign_in_otp.session_not_saved"));
    }

    let _ = session.commit_transaction().await;
    info!(target:"sign_up_otp","ok");
    return ApiResponse::ok(
        AuthOTPResponse {
            token: create_session_token.unwrap(),
        },
        &i18n.translate("sign_up_otp.ok"),
    );
}

pub async fn sign_in_otp(
    mut state: State<AppState>,
    i18n: i18n::I18n,
    user: UserDTO,
    body: AuthOTPRequest,
) -> ApiResponse<AuthOTPResponse> {
    let allowed = [USER_STATUS_ACTIVE, USER_STATUS_WAITING_ACTIVATION];

    let is_allowed = allowed.into_iter().any(|value| {
        value.to_string()
            == user
                .status
                .clone()
                .unwrap_or(USER_STATUS_WAITING_ACTIVATION.to_string())
    });
    if !is_allowed {
        info!(target:"sign_in_otp","user status not allowed {:?}",user.status);
        return ApiResponse::un_authorized(&i18n.translate("sign_in_otp.status_not_allowed"));
    }

    let create_user_id = user.id.clone().unwrap().to_string();
    let session = state.db.start_session().await;
    if let Err(err) = session {
        info!(target:"sign_in_otp","{:?}",err);
        return ApiResponse::un_authorized(&i18n.translate("sign_in_otp.session_invalid"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let save_user = DB::update(COLLECTION_USERS)
        .set(doc! {
            "last_logged_id":DateTime::now()
        })
        .filter(vec![is("_id", user.id)])
        .execute_with_session(&state.db, &mut session)
        .await;

    if let Err(why) = save_user {
        info!(target:"sign_in_otp","{:?}",why);
        let _ = session.abort_transaction().await;
        return ApiResponse::un_authorized(&i18n.translate("sign_in_otp.failed_update_user"));
    }
    let session_id = ObjectId::new().to_string();

    let create_session_token =
        JwtUtil::encode(session_id.clone(), REDIS_SESSION_OTP_SIGN_IN.to_string());
    if let None = create_session_token {
        info!(target:"sign_in_otp","failed to create session token");
        let _ = session.abort_transaction().await;
        return ApiResponse::un_authorized(&i18n.translate("sign_in_otp.session_not_created"));
    }

    let create_otp = generate_otp();
    let created_at = chrono::Local::now().naive_local().and_utc().timestamp();

    let send_otp = wa::send_otp(body.phone_number.clone(), create_otp.clone()).await;

    if let Err(why) = send_otp {
        info!(target:"sign_in_otp","failed to send otp {:?}",why);
        let _ = session.abort_transaction().await;
        return ApiResponse::un_authorized(&i18n.translate("sign_in_otp.otp_not_sent"));
    }

    let create_otp_session = &state.redis.set_session_otp(
        &session_id,
        &[
            (REDIS_KEY_USER_ID, create_user_id.to_string()),
            (REDIS_KEY_USER_TOKEN, create_session_token.clone().unwrap()),
            (REDIS_KEY_OTP_PHONE_NUMBER, body.phone_number.clone()),
            (REDIS_KEY_OTP, create_otp),
            (REDIS_KEY_OTP_TYPE, REDIS_SESSION_OTP_SIGN_IN.to_string()),
            (REDIS_KEY_OTP_AT, created_at.to_string()),
            (REDIS_KEY_OTP_ATTEMPT, "0".to_string()),
        ],
    );

    if let Err(why) = create_otp_session {
        info!(target:"sign_in_otp","failed to save session {:?}",why);
        let _ = session.abort_transaction().await;
        return ApiResponse::un_authorized(&i18n.translate("sign_in_otp.session_not_saved"));
    }

    let _ = session.commit_transaction().await;
    info!(target:"sign_in_otp","ok");
    return ApiResponse::ok(
        AuthOTPResponse {
            token: create_session_token.unwrap(),
        },
        &i18n.translate("sign_in_otp.ok"),
    );
}

pub async fn verify_otp(
    mut state: State<AppState>,
    lang: Lang,
    claims: JwtClaims,
    Json(body): Json<VerifyOTPRequest>,
) -> ApiResponse<VerifyOTPResponse> {
    let i18n = i18n!("auth", lang);
    info!(target:"verify_otp","starting...");

    let validate = body.validate();
    if let Err(err) = validate {
        info!(target:"verify_otp","{:?}",err);
        return ApiResponse::error_validation(err, &i18n.translate("verify_otp.validation_error"));
    }

    let get_session = state.redis.get_session_otp(&claims.sub);
    if let Err(why) = get_session {
        info!(target:"verify_otp","session invalid {:?}",why);
        return ApiResponse::un_authorized(&i18n.translate("verify_otp.failed"));
    }

    let session = get_session.unwrap();
    let session_type = get_string_with_default(session.get(REDIS_KEY_OTP_TYPE));
    let session_otp_at = get_naive_date_time(session.get(REDIS_KEY_OTP_AT));
    let session_otp = get_string_with_default(session.get(REDIS_KEY_OTP));
    let mut session_attempt = get_i64_with_default(session.get(REDIS_KEY_OTP_ATTEMPT));
    let session_user_id = get_string_with_default(session.get(REDIS_KEY_USER_ID));
    let session_user_object_id = ObjectId::from_str(&session_user_id.clone());

    if let Err(why) = session_user_object_id {
        info!(target:"verify_otp","session user id is not valid ObjectId value:{} err:{}",session_user_id,why);
        return ApiResponse::un_authorized(&i18n.translate("verify_otp.invalid_user_id"));
    }
    let session_user_object_id = session_user_object_id.unwrap();

    let find_user = DB::get(COLLECTION_USERS)
        .filter(vec![is("_id", session_user_object_id)])
        .get_one::<UserDTO>(&state.db)
        .await;

    if let Err(why) = find_user {
        info!(target:"verify_otp","user not exist {:?}",why);
        return ApiResponse::un_authorized(&i18n.translate("verify_otp.user_not_exist"));
    }
    let user = find_user.unwrap();

    if session_attempt >= 3 {
        info!(target:"verify_otp","max attempt reached attempt is: {}",session_attempt);
        return ApiResponse::un_authorized(&i18n.translate("verify_otp.max_attempt"));
    }

    let current_time = Local::now().naive_local().and_utc();
    let duration = current_time - session_otp_at;
    if duration.num_minutes() >= 2 {
        info!(target:"verify_otp","duration more than 3 minutes duration: {:?} minutes current:{:?} otp at: {:?}",duration.num_minutes(),current_time,session_otp_at);
        session_attempt += 1;
        let _update_attempt = state.redis.set_session_otp(
            &claims.sub,
            &[(REDIS_KEY_OTP_ATTEMPT, session_attempt.to_string())],
        );
        return ApiResponse::un_authorized(&i18n.translate("verify_otp.expired"));
    }

    //make sure the session is from previous auth otp
    if claims.provider != session_type.clone() {
        info!(target:"verify_otp","session and token did not match : provider is {} but session is {}",claims.provider,session_type);
        session_attempt += 1;
        let _update_attempt = state.redis.set_session_otp(
            &claims.sub,
            &[(REDIS_KEY_OTP_ATTEMPT, session_attempt.to_string())],
        );
        return ApiResponse::un_authorized(&i18n.translate("verify_otp.invalid_type"));
    }

    //make sure the otp is valid
    if session_otp != body.otp {
        info!(target:"verify_otp","otp did not match request:{} session:******",body.otp);
        session_attempt += 1;
        let _update_attempt = state.redis.set_session_otp(
            &claims.sub,
            &[(REDIS_KEY_OTP_ATTEMPT, session_attempt.to_string())],
        );
        return ApiResponse::un_authorized(&i18n.translate("verify_otp.invalid_type"));
    }

    let session = state.db.start_session().await;
    if let Err(err) = session {
        info!(target:"sign_in_otp","{:?}",err);
        return ApiResponse::un_authorized(&i18n.translate("sign_in_otp.session_invalid"));
    }
    let mut session = session.unwrap();
    let _ = session.start_transaction().await;

    let update_user = DB::update(COLLECTION_USERS)
        .filter(vec![is("_id", session_user_object_id)])
        .set(doc! {
            "last_logged_in":DateTime::now(),
            "status":USER_STATUS_ACTIVE
        })
        .execute_with_session(&state.db, &mut session)
        .await;

    if let Err(why) = update_user {
        info!(target:"verify_otp","failed to update user {}",why);
        let _ = session.abort_transaction().await;
        return ApiResponse::un_authorized(&i18n.translate("verify_otp.user_not_updated"));
    }

    let _remove_session = state.redis.delete_session_otp(&claims.sub);

    let create_session_sign_in = ObjectId::new().to_string();
    let create_token = JwtUtil::encode(
        create_session_sign_in.clone(),
        REDIS_SESSION_SIGN_IN.to_string(),
    );

    let save_sign_in_session = state.redis.set_session_sign_in(
        &create_session_sign_in,
        &[
            (REDIS_KEY_USER_ID, user.id.clone().unwrap().to_string()),
            (REDIS_KEY_USER_TOKEN, create_token.clone().unwrap()),
            (REDIS_KEY_USER_PHONE_NUMBER, user.phone_number.clone()),
        ],
    );

    if let Err(why) = save_sign_in_session {
        info!(target:"verify_otp","failed to update user {}",why);
        let _ = session.abort_transaction().await;
        return ApiResponse::un_authorized(&i18n.translate("verify_otp.user_not_updated"));
    }
    let _ = session.commit_transaction().await;
    ApiResponse::ok(
        VerifyOTPResponse {
            token: create_token.unwrap(),
            storage_token: "".to_string(),
            account: user,
            auth_type: claims.provider,
        },
        &i18n.translate(""),
    )
}

pub async fn resend_otp(
    mut state: State<AppState>,
    lang: Lang,
    claims: JwtClaims,
) -> ApiResponse<String> {
    let i18n = i18n!("auth", lang);
    info!(target:"resend_otp","starting...");

    let get_session = state.redis.get_session_otp(&claims.sub);
    if let Err(why) = get_session {
        info!(target:"resend_otp","session invalid {:?}",why);
        return ApiResponse::un_authorized(&i18n.translate("resend_otp.failed"));
    }

    let session = get_session.unwrap();
    let session_type = get_string_with_default(session.get(REDIS_KEY_OTP_TYPE));
    let session_otp_at = get_naive_date_time(session.get(REDIS_KEY_OTP_AT));
    let session_attempt = get_i64_with_default(session.get(REDIS_KEY_OTP_ATTEMPT));
    let session_user_id = get_string_with_default(session.get(REDIS_KEY_USER_ID));
    let session_phone_number = get_string_with_default(session.get(REDIS_KEY_OTP_PHONE_NUMBER));
    let session_user_object_id = ObjectId::from_str(&session_user_id.clone());

    if let Err(why) = session_user_object_id {
        info!(target:"resend_otp","session user id is not valid ObjectId value:{} err:{}",session_user_id,why);
        return ApiResponse::un_authorized(&i18n.translate("resend_otp.invalid_user_id"));
    }
    let session_user_object_id = session_user_object_id.unwrap();

    let find_user = DB::get(COLLECTION_USERS)
        .filter(vec![is("_id", session_user_object_id)])
        .get_one::<UserDTO>(&state.db)
        .await;

    if let Err(why) = find_user {
        info!(target:"resend_otp","user not exist {:?}",why);
        return ApiResponse::un_authorized(&i18n.translate("resend_otp.user_not_exist"));
    }
    let user = find_user.unwrap();

    let allowed = [USER_STATUS_ACTIVE, USER_STATUS_WAITING_ACTIVATION];
    let is_allowed = allowed.into_iter().any(|value| {
        value.to_string()
            == user
                .status
                .clone()
                .unwrap_or(USER_STATUS_WAITING_ACTIVATION.to_string())
    });
    if !is_allowed {
        info!(target:"resend_otp","user status not allowed {:?}",user.status);
        return ApiResponse::un_authorized(&i18n.translate("resend_otp.status_not_allowed"));
    }

    if session_attempt >= 3 {
        info!(target:"resend_otp","max attempt reached attempt is: {}",session_attempt);
        return ApiResponse::un_authorized(&i18n.translate("resend_otp.max_attempt"));
    }

    let current_time = Local::now().naive_local().and_utc();
    let duration = current_time - session_otp_at;
    if duration.num_minutes() <= 2 {
        info!(target:"resend_otp","duration less than 3 minutes duration: {:?} minutes",duration.num_minutes());
        return ApiResponse::un_authorized(&i18n.translate("resend_otp.waiting"));
    }

    //make sure the session is from previous auth otp
    if claims.provider != session_type.clone() {
        info!(target:"resend_otp","session and token did not match : provider is {} but session is {}",claims.provider,session_type);
        return ApiResponse::un_authorized(&i18n.translate("resend_otp.invalid_type"));
    }

    let generate_new_otp = generate_otp();
    let created_at = chrono::Local::now().naive_local().and_utc().timestamp();

    let send_otp = wa::send_otp(session_phone_number, generate_new_otp.clone()).await;

    if let Err(why) = send_otp {
        info!(target:"resend_otp","failed to send otp {:?}",why);
        return ApiResponse::un_authorized(&i18n.translate("sign_in_otp.otp_not_sent"));
    }

    let update_otp_session = state.redis.set_session_otp(
        &claims.sub,
        &[
            (REDIS_KEY_OTP, generate_new_otp.clone()),
            (REDIS_KEY_OTP_AT, format!("{}", created_at)),
        ],
    );
    if let Err(why) = update_otp_session {
        info!(target:"resend_otp","failed to update otp {}",why);
        return ApiResponse::un_authorized(&i18n.translate("resend_otp.user_not_updated"));
    }
    info!(target:"resend_otp","success");
    ApiResponse::ok("OK".to_string(), &i18n.translate("resend_otp.success"))
}
