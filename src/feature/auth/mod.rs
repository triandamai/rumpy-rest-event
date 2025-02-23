use crate::common::constant::{REDIS_KEY_USER_EMAIL, REDIS_KEY_USER_ID, REDIS_KEY_USER_TOKEN};
use crate::common::jwt::JwtUtil;
use crate::schema::tb_user::dsl::*;
use auth_model::{SignUpRequest, SignUpResponse};
use axum::extract::State;
use diesel::dsl::insert_into;
use diesel::{RunQueryDsl, SelectableHelper};
use log::info;
use serde_json::json;
use validator::Validate;

use crate::{
    common::{api_response::ApiResponse, app_state::AppState, lang::Lang, middleware::Json},
    entity::user::{CreateUser, User},
    i18n,
};

pub mod auth_model;

pub async fn sign_up_email(
    mut state: State<AppState>,
    lang: Lang,
    Json(body): Json<SignUpRequest>,
) -> ApiResponse<SignUpResponse> {
    let i18n = i18n!("auth", lang);

    //getting connection from pool
    let conn = state.postgres.get();
    if conn.is_err() {
        info!(target:"sign-up::email::failed","connection error {:?}",conn.err());
        return ApiResponse::failed(i18n.translate("sign-up.failed").as_str());
    }
    let mut conn = conn.unwrap();

    let validate = body.validate();
    if validate.is_err() {
        let errors = validate.unwrap_err();
        info!(target:"sign-up::email::failed","validation error {:?}",errors);
        return ApiResponse::error_validation(
            errors,
            i18n.translate("sign-up.validation.failed").as_str(),
        );
    }

    let create_password = bcrypt::hash(body.password, bcrypt::DEFAULT_COST);
    if create_password.is_err() {
        info!(target:"sign-up::email::failed","hash password error {:?}",create_password.unwrap_err());
        return ApiResponse::failed(i18n.translate("sign-up.failed").as_str());
    }
    let create_password = create_password.unwrap();
    let now = chrono::Utc::now().naive_local();
    //prepare inserting data
    let create_user = CreateUser {
        display_name: body.full_name,
        email: body.email.clone(),
        phone_number: None,
        password: Some(create_password),
        app_meta_data: None,
        user_meta_data: Some(json!({"providers":["BASIC"]})),
        created_at: Some(now.clone()),
        updated_at: Some(now.clone()),
        confirmation_at: None,
        confirmation_sent_at: None,
    };

    let insert_data = insert_into(tb_user)
        .values(create_user)
        .returning(User::as_returning())
        .get_result::<User>(&mut conn);

    if insert_data.is_err() {
        info!(target:"sign-up::email::failed","insert user failed {:?}",insert_data.err());
        return ApiResponse::failed(i18n.translate("sign-up.failed").as_str());
    }
    let insert_data = insert_data.unwrap();

    let create_token = JwtUtil::encode(body.email.clone());
    if create_token.is_none() {
        info!(target:"sign-up::email::failed","create token error {:?}",create_token);
        return ApiResponse::failed(i18n.translate("sign-up.failed").as_str());
    }
    let create_token = create_token.unwrap();

    let save_token_to_redis = state.redis.set_session_sign_up(
        body.email.clone().as_str(),
        &[
            (REDIS_KEY_USER_TOKEN, create_token.clone()),
            (REDIS_KEY_USER_EMAIL, body.email),
            (REDIS_KEY_USER_ID, format!("{}", insert_data.id)),
        ],
    );
    if save_token_to_redis.is_err() {
        info!(target:"sign-up::email::failed","save token error {:?}",save_token_to_redis.unwrap_err());
        return ApiResponse::failed(i18n.translate("sign-up.failed").as_str());
    }

    //todo:: send verification email

    ApiResponse::ok(
        SignUpResponse {
            token: create_token,
        },
        i18n.translate("sign-up.success").as_str(),
    )
}
