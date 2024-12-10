use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::orm::orm::Orm;
use crate::common::utils::create_object_id_option;
use crate::dto::coach::CoachDTO;
use crate::entity::coach::Coach;
use crate::feature::coach::coach_model::{CreateCoachRequest, UpdateCoachRequest};
use crate::translate;
use axum::extract::{Path, Query, State};
use axum::Json;
use bson::oid::ObjectId;
use bson::DateTime;
use std::str::FromStr;
use validator::Validate;

pub async fn get_list_coach(
    mut state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<Coach>> {
    if auth_context.authorize("app::coach::read") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let find_coach = Orm::get("coach")
        .pageable::<Coach>(
            query.page.unwrap_or(1),
            query.size.unwrap_or(100),
            &state.db,
        )
        .await;

    if find_coach.is_err() {
        return ApiResponse::failed(translate!("", lang).as_str());
    }
    return ApiResponse::ok(find_coach.unwrap(), translate!("", lang).as_str());
}

pub async fn get_detail_coach(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    Path(coach_id): Path<String>,
) -> ApiResponse<Coach> {
    if !auth_context.authorize("app::coach::read") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }
    let coach_id = ObjectId::from_str(coach_id.as_str());
    if coach_id.is_err() {
        return ApiResponse::not_found(translate!("", lang).as_str());
    }
    let find_coach = Orm::get("coach")
        .filter_object_id("_id", &coach_id.unwrap())
        .one::<Coach>(&state.db)
        .await;

    if find_coach.is_err() {
        return ApiResponse::not_found(translate!("", lang).as_str());
    }
    ApiResponse::ok(find_coach.unwrap(), translate!("", lang).as_str())
}

pub async fn create_coach(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    body: Json<CreateCoachRequest>,
) -> ApiResponse<CoachDTO> {
    if !auth_context.authorize("app::coach::write") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }
    let validate = body.validate();
    if !validate.is_err() {
        return ApiResponse::failed(translate!("").as_str());
    }

    let mut coach = Coach {
        id: Some(ObjectId::new()),
        branch_id: create_object_id_option(&body.branch_id),
        full_name: body.coach_name.to_string(),
        email: body.coach_email.to_string(),
        phone_number: body.coach_phone_number.to_string(),
        gender: body.coach_gender.to_string(),
        created_by: None,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        deleted: false,
    };

    let save = Orm::insert("coach").one(&coach, &state.db).await;
    if save.is_err() {
        return ApiResponse::failed(translate!("").as_str());
    }

    ApiResponse::ok(coach.to_dto(), translate!("").as_str())
}

pub async fn update_coach(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    body: Json<UpdateCoachRequest>,
) -> ApiResponse<CoachDTO> {
    if !auth_context.authorize("app::coach::write") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }
    let validate = body.validate();
    if !validate.is_err() {
        return ApiResponse::failed(translate!("").as_str());
    }
    let coach_id = ObjectId::from_str(body.coach_id.as_str());
    if coach_id.is_err() {
        return ApiResponse::failed(translate!("").as_str());
    }
    let coach_id = coach_id.unwrap();
    let find_coach = Orm::get("coach")
        .filter_object_id("_id", &coach_id)
        .one::<Coach>(&state.db)
        .await;

    if find_coach.is_err() {
        return ApiResponse::failed(translate!("").as_str());
    }
    let mut coach = find_coach.unwrap();

    let update = Orm::update("coach")
        .filter_object_id("_id", &coach_id)
        .one(&coach, &state.db)
        .await;

    if update.is_err() {
        return ApiResponse::failed(translate!("").as_str());
    }

    ApiResponse::ok(coach.to_dto(), translate!("").as_str())
}
