use crate::common::constant::{COLLECTION_USER, PROVIDER_BASIC, REDIS_KEY_USER_EMAIL, REDIS_KEY_USER_ID, REDIS_KEY_USER_TOKEN};
use crate::common::jwt::{JwtClaims, JwtUtil};
use crate::entity::user::User;
use crate::entity::user_metadata::UserMetaData;
use auth_model::{SignInEmailRequest, SignInResponse, SignUpRequest};
use axum::extract::{Path, State};
use bcrypt::DEFAULT_COST;
use bson::oid::ObjectId;
use bson::{doc, DateTime};
use log::info;
use std::ptr::eq;
use validator::Validate;

use crate::common::mongo::filter::{equal, is};
use crate::common::mongo::DB;
use crate::feature::auth::auth_model::{
    ResetPasswordRequest, SetNewPasswordRequest, VerifyResetPasswordResponse,
};
use crate::{
    common::{api_response::ApiResponse, app_state::AppState, lang::Lang, middleware::Json},
    i18n,
};

pub mod auth_model;

pub async fn sign_up_email(
    mut state: State<AppState>,
    lang: Lang,
    Json(body): Json<SignUpRequest>,
) -> ApiResponse<String> {
    let i18n = i18n!("auth", lang);

    let validate = body.validate();
    if let Err(err) = validate {
        info!(target:"sign-up::email::failed","validation error {:?}",err);
        return ApiResponse::error_validation(
            err,
            i18n.translate("sign-up.validation.failed").as_str(),
        );
    }

    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![equal("email",  body.email.clone())])
        .get_one::<User>(&state.db)
        .await;

    if let Ok(_user) = find_user {
        info!(target:"sign-up::email::failed","email already in use");
        return ApiResponse::failed(i18n.translate("sign-up.email.in-use").as_str());
    }

    let create_password = bcrypt::hash(body.password, bcrypt::DEFAULT_COST);
    if let Err(err) = create_password {
        info!(target:"sign-up::email::failed","hash password error {:?}",err);
        return ApiResponse::failed(i18n.translate("sign-up.failed").as_str());
    }
    let create_password = create_password.unwrap();
    let now = DateTime::now();
    //prepare inserting data
    let create_user = User {
        id: Some(ObjectId::new()),
        display_name: body.full_name,
        email: body.email.clone(),
        phone_number: None,
        password: Some(create_password),
        app_meta_data: None,
        user_meta_data: Some(UserMetaData {
            providers: Some(vec![PROVIDER_BASIC.to_string()]),
        }),
        profile_picture: None,
        created_at: now,
        updated_at: now,
        confirmation_at: None,
        confirmation_sent_at: Some(now),
    };

    let insert_data = DB::insert(COLLECTION_USER).one(create_user, &state.db).await;

    if let Err(err) = insert_data {
        info!(target:"sign-up::email::failed","insert user failed {:?}",err);
        return ApiResponse::failed(i18n.translate("sign-up.failed").as_str());
    }
    let insert_data = insert_data.unwrap();
    let session_id = ObjectId::new().to_string();

    let save_token_to_redis = state.redis.set_session_sign_up(
        session_id.clone().as_str(),
        &[
            (REDIS_KEY_USER_EMAIL, body.email),
            (REDIS_KEY_USER_ID, format!("{}", insert_data.to_string())),
        ],
    );
    if let Err(err) = save_token_to_redis {
        info!(target:"sign-up::email::failed","save token error {:?}",err);
        return ApiResponse::failed(i18n.translate("sign-up.failed").as_str());
    }

    //todo:: send verification email

    ApiResponse::ok(
        String::from("OK"),
        i18n.translate("sign-up.success").as_str(),
    )
}

pub async fn sign_up_email_confirmation(
    mut state: State<AppState>,
    lang: Lang,
    Path(code): Path<String>,
) -> ApiResponse<String> {
    let i18n = i18n!("auth", lang);

    let get_session = state.redis.get_session_sign_up(code.as_str());
    if let Err(err) = get_session {
        info!(target:"sign-up::verify-email::failed","connection error {:?}",err);
        return ApiResponse::failed(i18n.translate("sign-up.email.confirmation.expire").as_str());
    }
    let session = get_session.unwrap();
    let user_email = session.get(REDIS_KEY_USER_EMAIL);
    if let None = user_email {
        info!(target:"sign-up::verify-email::failed","redis session not contain email");
        return ApiResponse::failed(i18n.translate("sign-up.email.confirmation.expire").as_str());
    }
    let user_email = user_email.unwrap();

    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![equal("email", user_email)])
        .get_one::<User>(&state.db)
        .await;

    if let Err(err) = find_user {
        info!(target:"sign-up::verify-email::failed","{:?}",err);
        return ApiResponse::failed(i18n.translate("sign-up.email.confirmation.failed").as_str());
    }

    let user = find_user.unwrap();

    let update_user = DB::update(COLLECTION_USER)
        .set(doc! {
            "updated_at":DateTime::now(),
            "confirmation_at": DateTime::now()
        })
        .filter(vec![is("_id",&user.id.unwrap())])
        .execute(&state.db)
        .await;

    if let Err(err) = update_user {
        info!(target:"sign-up::verify-email::failed","{:?}",err);
        return ApiResponse::failed(i18n.translate("sign-up.email.confirmation.failed").as_str());
    }

    let _remove_session = state.redis.delete_session_sign_up(&code);

    ApiResponse::ok(
        "".to_string(),
        &i18n.translate("sign-up.email.confirmation.success"),
    )
}

pub async fn sign_in_email(
    mut state: State<AppState>,
    lang: Lang,
    Json(body): Json<SignInEmailRequest>,
) -> ApiResponse<SignInResponse> {
    let i18n = i18n!("auth", lang);

    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![equal("email", &body.email)])
        .get_one::<User>(&state.db)
        .await;

    if let Err(err) = find_user {
        info!(target:"sign-in::email::failed","connection error {:?}",err);
        return ApiResponse::failed(i18n.translate("sign-in.email.user.not-found").as_str());
    }
    let user = find_user.unwrap();

    if let None = user.confirmation_at {
        info!(target:"sign-in::email::failed","user not verified ");
        return ApiResponse::failed(i18n.translate("sign-in.email.user.not-confirmed").as_str());
    }
    let meta_data = user
        .user_meta_data
        .clone()
        .unwrap_or(UserMetaData { providers: None });
    let providers = meta_data.providers.unwrap_or(Vec::new());
    //check whether user provider BASIC(email)
    if !providers.contains(&PROVIDER_BASIC.to_string()) {
        info!(target:"sign-in::email::failed","user doesn't have BASIC provider current:{:?}",providers);
        return ApiResponse::failed(
            i18n.translate("sign-in.email.user.provider.not-found")
                .as_str(),
        );
    }
    if let None = user.password {
        info!(target:"sign-in::email::failed","user doesn't have BASIC password");
        return ApiResponse::failed(
            i18n.translate("sign-in.email.user.provider.not-found")
                .as_str(),
        );
    }
    let current_password = user.password.clone().unwrap();
    let verify_password = bcrypt::verify(body.password, &current_password);
    if let Err(err) = verify_password {
        info!(target:"sign-in::email::failed","password invalid {:?}",err);
        return ApiResponse::failed(
            i18n.translate("sign-in.email.user.invalid-credential")
                .as_str(),
        );
    }

    if !verify_password.unwrap() {
        info!(target:"sign-in::email::failed","password invalid");
        return ApiResponse::failed(
            i18n.translate("sign-in.email.user.invalid-credential")
                .as_str(),
        );
    }

    let session_id = user.email.clone();

    let create_token = JwtUtil::encode(session_id.clone());
    if let None = create_token {
        info!(target:"sign-in::email::failed","create token error {:?}",create_token);
        return ApiResponse::failed(i18n.translate("sign-in.email.failed").as_str());
    }
    let create_token = create_token.unwrap();

    let save_token_to_redis = state.redis.set_session_sign_in(
        session_id.clone().as_str(),
        &[
            (REDIS_KEY_USER_TOKEN, create_token.clone()),
            (REDIS_KEY_USER_EMAIL, user.email.clone()),
            (
                REDIS_KEY_USER_ID,
                format!("{}", user.id.unwrap().to_string()),
            ),
        ],
    );

    if let Err(err) = save_token_to_redis {
        info!(target:"sign-in::email::failed","save token error {:?}",err);
        return ApiResponse::failed(i18n.translate("sign-in.email.failed").as_str());
    }

    ApiResponse::ok(
        SignInResponse {
            token: create_token,
            account: user.into(),
        },
        &i18n.translate("sign-in.email.success").as_str(),
    )
}

pub async fn request_reset_password(
    mut state: State<AppState>,
    lang: Lang,
    Json(body): Json<ResetPasswordRequest>,
) -> ApiResponse<String> {
    let i18n = i18n!("auth", lang);

    let validate = body.validate();
    if let Err(err) = validate {
        info!(target:"reset-password::validate::failed","validation error {}",err);
        return ApiResponse::error_validation(
            err,
            i18n.translate("forgot.password.validation.error").as_str(),
        );
    }

    let find_user = DB::get(COLLECTION_USER)
        .filter(vec![equal("email", &body.email)])
        .get_one::<User>(&state.db)
        .await;
    if let Err(err) = find_user {
        info!(target:"reset-password::request::failed","connection error {:?}",err);
        return ApiResponse::failed(i18n.translate("forgot.password.request.failed").as_str());
    }

    let user = find_user.unwrap();

    let session_id = ObjectId::new().to_string();
    let save_session = state.redis.set_session_reset_password(
        session_id.clone().as_str(),
        &[(REDIS_KEY_USER_EMAIL, user.email.clone())],
    );
    if let Err(err) = save_session {
        info!(target:"reset-password::request::failed","connection error {:?}",err);
        return ApiResponse::failed(i18n.translate("forgot.password.request.failed").as_str());
    }

    //todo:: send email

    //
    ApiResponse::ok(
        "OK".to_string(),
        i18n.translate("forgot.password.request.success").as_str(),
    )
}

pub async fn verify_reset_password(
    mut state: State<AppState>,
    lang: Lang,
    Path(code): Path<String>,
) -> ApiResponse<VerifyResetPasswordResponse> {
    let i18n = i18n!("auth", lang);

    let get_session = state
        .redis
        .get_session_reset_password(code.clone().as_str());

    if let Err(err) = get_session {
        info!(target:"reset-password::request::failed","connection error {:?}",err);
        return ApiResponse::failed(i18n.translate("reset-password.failed").as_str());
    }

    let session = get_session.unwrap();
    let user_email = session.get(REDIS_KEY_USER_EMAIL);

    if let None = user_email {
        info!(target:"reset-password::request::failed","connection error ");
        return ApiResponse::failed(i18n.translate("reset-password.failed").as_str());
    }

    let user_email = user_email.unwrap();

    let create_token = JwtUtil::encode(user_email.clone());
    if let None = create_token {
        info!(target:"reset-password::request::failed","connection error ");
        return ApiResponse::failed(i18n.translate("reset-password.failed").as_str());
    }

    let token = create_token.unwrap();
    let _remove_session = state
        .redis
        .delete_session_reset_password(code.clone().as_str());

    let save_session = state.redis.set_session_reset_password(
        user_email.clone().as_str(),
        &[(REDIS_KEY_USER_EMAIL, user_email.clone())],
    );

    if let Err(err) = save_session {
        info!(target:"reset-password::request::failed","{:?}",err);
        return ApiResponse::failed(i18n.translate("reset-password.failed").as_str());
    }
    ApiResponse::ok(
        VerifyResetPasswordResponse { token },
        i18n.translate("reset-password.request.success").as_str(),
    )
}

pub async fn set_new_password(
    mut state: State<AppState>,
    lang: Lang,
    jwt: JwtClaims,
    Json(body): Json<SetNewPasswordRequest>,
) -> ApiResponse<String> {
    let i18n = i18n!("auth", lang);

    let get_session = state
        .redis
        .get_session_reset_password(jwt.sub.clone().as_str());
    if let Err(err) = get_session {
        info!(target:"auth::request::failed","connection error {:?}",err);
        return ApiResponse::failed(
            i18n.translate("forgot.password.set-password.expire")
                .as_str(),
        );
    }

    let session = get_session.unwrap();
    let user_email = session.get(REDIS_KEY_USER_EMAIL);
    if let None = user_email {
        info!(target:"auth::request::failed","connection error ");
        return ApiResponse::failed(
            i18n.translate("forgot.password.set-password.expire")
                .as_str(),
        );
    }
    let user_email = user_email.unwrap();

    let create_password = bcrypt::hash(body.new_password, DEFAULT_COST);
    if let Err(err) = create_password {
        info!(target:"auth::request::failed","connection error {:?}",err);
        return ApiResponse::failed(&i18n.translate("forgot.password.set-password.failed"), );
    }

    let update_password = DB::update(COLLECTION_USER)
        .set(doc! {
            "password":create_password.unwrap()
        })
        .filter(vec![equal("email", &user_email)])
        .execute(&state.db)
        .await;
    if let Err(err) = update_password {
        info!(target:"auth::request::failed","connection error {:?}",err);
        return ApiResponse::failed(&i18n.translate("forgot.password.set-password.failed"), );
    }

    let _remove_session = state
        .redis
        .delete_session_reset_password(jwt.sub.clone().as_str());

    ApiResponse::ok(
        "OK".to_string(),
        &i18n.translate("forgot.password.set-password.success"),
    )
}
