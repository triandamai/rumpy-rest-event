use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::jwt::{AuthContext, JwtClaims, JwtUtil};
use crate::common::lang::Lang;
use crate::common::orm::orm::Orm;
use crate::common::smtp::SmtpClient;
use crate::entity::account::Account;
use crate::entity::account_permission::AccountPermission;
use crate::feature::auth::auth_model::{ChangePasswordRequest, CompleteForgotPasswordRequest, ForgotPasswordRequest, SignInStaffRequest, SignInStaffResponse, ISSUED_AT_KEY, TOKEN_KEY, USER_ID_KEY};
use crate::translate;
use axum::extract::State;
use axum::Json;
use bcrypt::DEFAULT_COST;
use bson::oid::ObjectId;
use chrono::Utc;
use std::str::FromStr;
use validator::Validate;

pub async fn sign_in(
    mut state: State<AppState>,
    lang: Lang,
    body: Json<SignInStaffRequest>,
) -> ApiResponse<SignInStaffResponse> {
    let validate = body.validate();
    if !validate.is_ok() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation.error", lang.get()).as_str(),
        );
    }

    let find_by_email = Orm::get("account")
        .filter_string("email", Some("$eq"), body.email.as_str())
        .one::<Account>(&state.db)
        .await;

    if find_by_email.is_err() {
        return ApiResponse::failed(translate!("sign-in.failed", lang.get()).as_str());
    }
    let find = find_by_email.unwrap();
    let verify = bcrypt::verify(body.password.as_str(), &find.password).unwrap_or(false);

    if !verify {
        return ApiResponse::failed(translate!("sign-in.failed", lang.get()).as_str());
    }
    let create_token = JwtUtil::encode(find.email.clone());
    if create_token.is_none() {
        return ApiResponse::failed(translate!("sign-in.failed", lang.get()).as_str());
    }
    let token = create_token.unwrap();
    let save_session = state.redis.set_session_sign_in(
        find.email.as_str(),
        &[
            (USER_ID_KEY, find.id.unwrap().to_string()),
            (TOKEN_KEY, token.clone()),
        ],
    );
    if save_session.is_err() {
        return ApiResponse::failed(translate!("sign-in.failed", lang.get()).as_str());
    }

    let find_permission = Orm::get("account-permission")
        .filter_object_id("user_id", &find.id.unwrap())
        .all::<AccountPermission>(&state.db)
        .await
        .unwrap_or(Vec::new())
        .iter()
        .map(|e| (e.name.clone(), e.value.clone()))
        .collect::<Vec<(String, String)>>();

    let _save_permission = state
        .redis
        .set_session_permission(find.email.as_str(), find_permission);

    ApiResponse::ok(
        SignInStaffResponse { token },
        translate!("sign-in.success", lang.get()).as_str(),
    )
}

pub async fn change_password(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    body: Json<ChangePasswordRequest>,
) -> ApiResponse<String> {
    let validate = body.validate();
    if !validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation.error", lang.get()).as_str(),
        );
    }
    let find_by_email = Orm::get("account")
        .filter_string("email", Some("$eq"), auth_context.claims.sub.as_str())
        .one::<Account>(&state.db)
        .await;
    if find_by_email.is_err() {
        return ApiResponse::failed(translate!("validation.error", lang.get()).as_str());
    }
    let mut find = find_by_email.unwrap();
    let verify_current_password = bcrypt::verify(body.current_password.as_str(), &find.password);
    if verify_current_password.is_err() {
        return ApiResponse::failed(translate!("validation.error", lang.get()).as_str());
    }
    let create_new_password = bcrypt::hash(body.new_password.as_str(), DEFAULT_COST);
    if create_new_password.is_err() {
        return ApiResponse::failed(translate!("validation.error", lang.get()).as_str());
    }
    find.password = create_new_password.unwrap();
    let update = Orm::update("account")
        .filter_object_id("_id", &find.id.clone().unwrap())
        .one(find, &state.db)
        .await;
    if update.is_err() {
        return ApiResponse::failed(translate!("validation.error", lang.get()).as_str());
    }
    ApiResponse::ok(
        "OK".to_string(),
        translate!("change-password-success", lang.get()).as_str(),
    )
}

pub async fn request_forgot_password(
    state: State<AppState>,
    lang: Lang,
    auth_context: JwtClaims,
    body: Json<ForgotPasswordRequest>,
) -> ApiResponse<String> {
    let validate = body.validate();
    if !validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation.error", lang.get()).as_str(),
        );
    }
    let find_by_email = Orm::get("account")
        .filter_string("email", Some("$eq"), auth_context.sub.as_str())
        .one::<Account>(&state.db)
        .await;
    if find_by_email.is_err() {
        return ApiResponse::failed(translate!("validation.error", lang.get()).as_str());
    }

    let mut find = find_by_email.unwrap();

    let save_forgot_password_session = &state.redis.set_session_forgot_password(
        body.email.as_str(),
        &[
            (USER_ID_KEY, find.id.unwrap().to_string()),
            (ISSUED_AT_KEY, Utc::now().timestamp().to_string()),
        ],
    );
    if save_forgot_password_session.is_err() {
        return ApiResponse::failed(translate!("validation.error", lang.get()).as_str());
    }

    let _send_email = SmtpClient::new(&find.email)
        .send(&String::from("strong-teams@gmail.com"), &String::from(""))
        .await;
    ApiResponse::ok(
        "".to_string(),
        translate!("change-password-success", lang.get()).as_str(),
    )
}

pub async fn verify_token_forgot_password(
    state: State<AppState>,
    lang: Lang,
    auth_context: JwtClaims,
) -> ApiResponse<String> {
    let find_session = state
        .redis
        .get_session_forgot_password(auth_context.sub.as_str());
    if find_session.is_err() {
        return ApiResponse::failed(translate!("validation.error", lang.get()).as_str());
    }
    return ApiResponse::ok(
        "OK".to_string(),
        translate!("verify-token-success", lang.get()).as_str(),
    );
}

pub async fn complete_forgot_password(
    state: State<AppState>,
    lang: Lang,
    auth_context: JwtClaims,
    body: Json<CompleteForgotPasswordRequest>,
) -> ApiResponse<String> {
    let find_session = state
        .redis
        .get_session_forgot_password(auth_context.sub.as_str());
    if find_session.is_err() {
        return ApiResponse::failed(translate!("validation.error", lang.get()).as_str());
    }

    let user_id = find_session
        .unwrap()
        .get(USER_ID_KEY)
        .unwrap_or(&String::from(""));
    let user_id = ObjectId::from_str(user_id).unwrap_or(ObjectId::new());
    let find = Orm::get("account")
        .filter_object_id("_id", &user_id)
        .one::<Account>(&state.db)
        .await;
    if find.is_err() {
        return ApiResponse::failed(translate!("validation.error", lang.get()).as_str());
    }
    let mut find = find.unwrap();
    let hash_password = bcrypt::hash(
        body.new_password.as_str(),
        DEFAULT_COST,
    );
    if hash_password.is_err() {
        return ApiResponse::failed(translate!("validation.error", lang.get()).as_str());
    }
    let password = hash_password.unwrap();
    find.password = password;

    let update = Orm::update("account")
        .filter_object_id("_id", &find.id.unwrap())
        .set_str("password", find.password.as_str())
        .execute_one(&state.db)
        .await;

    if update.is_err() {
        return ApiResponse::failed(translate!("validation.error", lang.get()).as_str());
    }
    ApiResponse::ok(
        "OK".to_string(),
        translate!("complete-password-success", lang.get()).as_str(),
    )
}

pub async fn sign_out(
    mut state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
) -> ApiResponse<String> {
    let _remove_session = state
        .redis
        .delete_session_sign_in(auth_context.claims.sub.as_str());

    let _remove_permission = state
        .redis
        .delete_session_permission(auth_context.claims.sub.as_str());

    ApiResponse::ok(
        "OK".to_string(),
        translate!("sign.out.success", lang.get()).as_str(),
    )
}
