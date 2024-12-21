use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::jwt::{AuthContext, JwtUtil};
use crate::common::lang::Lang;
use crate::common::middleware::Json;
use crate::common::orm::orm::Orm;
use crate::dto::account_dto::AccountDetailDTO;
use crate::entity::account::Account;
use crate::feature::auth::auth_model::{
    ChangePasswordRequest, SignInRequest, SignInResponse, BRANCH_ID_KEY, TOKEN_KEY, USER_ID_KEY,
};
use crate::translate;
use axum::extract::State;
use bcrypt::DEFAULT_COST;
use log::info;
use validator::Validate;

pub async fn sign_in(
    mut state: State<AppState>,
    lang: Lang,
    Json(body): Json<SignInRequest>,
) -> ApiResponse<SignInResponse> {
    info!(target:"auth::sign-in", "trying to login");
    let validate = body.validate();
    if !validate.is_ok() {
        let err = validate.unwrap_err();
        info!(target:"auth::sign-in::error", "body invalid {:?}",err);
        return ApiResponse::error_validation(err, translate!("validation.error", lang));
    }

    let find_by_email = Orm::get("account")
        .filter_string("email", Some("$eq"), body.email.as_str())
        .one::<Account>(&state.db)
        .await;

    if find_by_email.is_err() {
        info!(target:"auth::sign-in::error", "{} trying to login, but cannot find account",body.email.as_str());
        return ApiResponse::failed(translate!("sign-in.account.not-found", lang));
    }
    let find = find_by_email.unwrap();
    let verify = bcrypt::verify(body.password.as_str(), &find.password).unwrap_or(false);

    if !verify {
        return ApiResponse::failed(translate!("sign-in.failed", lang));
    }
    let create_token = JwtUtil::encode(find.email.clone());
    if create_token.is_none() {
        info!(target:"auth::sign-in::error", "{} trying to login, but failed to create session", body.email.as_str());
        return ApiResponse::failed(translate!("sign-in.failed", lang));
    }
    let get_account = Orm::get("account")
        .filter_object_id("_id", &find.id.unwrap())
        .join_one("account", "reply_to_id", "_id", "report")
        .join_one("branch", "branch_id", "_id", "branch")
        .join_one("file-attachment", "_id", "ref_id", "profile_picture")
        .join_many("account-permission", "_id", "account_id", "permission")
        .one::<AccountDetailDTO>(&state.db)
        .await;
    if get_account.is_err() {
        info!(target:"auth::sign-in::error", "{} trying to login, but failed to fetch profile", body.email.as_str());
        return ApiResponse::failed(translate!("sign-in.failed", lang));
    }
    let get_account = get_account.unwrap();

    let token = create_token.unwrap();
    let save_session = state.redis.set_session_sign_in(
        find.email.as_str(),
        &[
            (USER_ID_KEY, find.id.unwrap().to_string()),
            (TOKEN_KEY, token.clone()),
            (BRANCH_ID_KEY, find.branch_id.unwrap().to_string()),
        ],
    );
    if save_session.is_err() {
        info!(target:"auth::sign-in::error", "{} trying to login, but failed to create session to redis", body.email.as_str());
        return ApiResponse::failed(translate!("sign-in.failed", lang).as_str());
    }

    let permission = get_account.clone().permission.unwrap_or(Vec::new());

    let find_permission = permission
        .iter()
        .map(|e| (e.value.clone(), e.value.clone()))
        .collect::<Vec<(String, String)>>();

    let _save_permission = state
        .redis
        .set_session_permission(find.email.as_str(), find_permission);

    info!(target:"auth::sign-in::success", "sign in success");
    ApiResponse::ok(
        SignInResponse {
            token,
            account: get_account,
        },
        translate!("sign-in.success", lang).as_str(),
    )
}

pub async fn change_password(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Json(body): Json<ChangePasswordRequest>,
) -> ApiResponse<String> {
    info!(target:"auth::auth", "{} change password ",auth_context.claims.sub);
    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation.error", lang).as_str(),
        );
    }

    let find_by_email = Orm::get("account")
        .filter_string("email", Some("$eq"), auth_context.claims.sub.as_str())
        .one::<Account>(&state.db)
        .await;

    if find_by_email.is_err() {
        return ApiResponse::failed(
            translate!("change-password.account.not-failed", lang).as_str(),
        );
    }
    let mut find = find_by_email.unwrap();
    let verify_current_password =
        bcrypt::verify(body.current_password.as_str(), &find.password).unwrap_or(false);
    if !verify_current_password {
        return ApiResponse::failed(translate!("change-password.password-invalid", lang).as_str());
    }
    let create_new_password = bcrypt::hash(body.new_password.as_str(), DEFAULT_COST);
    if create_new_password.is_err() {
        return ApiResponse::failed(
            translate!("change-password.new-password-invalid", lang).as_str(),
        );
    }
    find.password = create_new_password.unwrap();
    let update = Orm::update("account")
        .filter_object_id("_id", &find.id.clone().unwrap())
        .one(find, &state.db)
        .await;
    if update.is_err() {
        return ApiResponse::failed(translate!("change-password.failed", lang).as_str());
    }
    ApiResponse::ok(
        "OK".to_string(),
        translate!("change-password.success", lang).as_str(),
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
        translate!("sign-out.success", lang).as_str(),
    )
}
