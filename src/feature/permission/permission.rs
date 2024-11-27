use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::orm::orm::Orm;
use crate::common::utils::create_object_id_option;
use crate::entity::account_permission::AccountPermission;
use crate::entity::permission::Permission;
use crate::feature::permission::permission_model::AssignPermissionRequest;
use crate::translate;
use axum::extract::State;
use axum::Json;
use bson::oid::ObjectId;
use bson::DateTime;

pub async fn assign_permission(
    state: State<AppState>,
    lang: Lang,
    auth_context: AuthContext,
    body: Json<AssignPermissionRequest>,
) -> ApiResponse<AccountPermission> {
    if !auth_context.authorize("app::account::permission::write") {
        return ApiResponse::un_authorized(translate!("unauthorized", lang.get()).as_str());
    }

    let permission = Orm::get("permission")
        .filter_object_id(
            "_id",
            &create_object_id_option(body.permission_id.clone().as_str()).unwrap(),
        )
        .one::<Permission>(&state.db)
        .await;
    if permission.is_err() {
        return ApiResponse::not_found(translate!("permission.not-found", lang.get()).as_str());
    }

    let user_id = create_object_id_option(body.user_id.clone().as_str());
    let permission = permission.unwrap();

    let current_time = DateTime::now();
    let account_permission = AccountPermission {
        id: Some(ObjectId::new()),
        account_id: user_id,
        permission_id: permission.id,
        name: permission.name.clone(),
        value: permission.value.clone(),
        created_at: current_time,
        updated_at: current_time,
    };

    let exist = Orm::get("account-permission")
        .and()
        .filter_object_id(
            "permission_id",
            &account_permission.permission_id.clone().unwrap(),
        )
        .filter_object_id(
            "account_id",
            &account_permission.account_id.clone().unwrap(),
        )
        .one::<AccountPermission>(&state.db)
        .await;

    if exist.is_ok() {
        return ApiResponse::failed(translate!("user.permission.exist", lang.get()).as_str());
    }

    let saved = Orm::insert("account-permission")
        .one(&account_permission, &state.db)
        .await;

    if saved.is_err() {
        return ApiResponse::failed(translate!("user.permission.exist", lang.get()).as_str());
    }

    ApiResponse::ok(
        account_permission,
        translate!("permission.saved", "account-permission").as_str(),
    )
}
