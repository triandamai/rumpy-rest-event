use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::entity::user_credential::{AuthProvider, UserCredential, UserDTO, UserStatus};
use axum::extract::State;
use mongodb::bson::DateTime;

pub async fn index(state: State<AppState>) -> ApiResponse<UserDTO> {
    let mut create_user = UserCredential {
        id: None,
        full_name: "Trian Damai".to_string(),
        email: "triandamai@gmail.com".to_string(),
        password: "password".to_string(),
        status: UserStatus::WaitingConfirmation,
        date_of_birth: None,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        username: "triandamai".to_string(),
        deleted: false,
        auth_provider: AuthProvider::Basic,
    };
    let data = create_user
        .save(&state.db)
        .await;

    ApiResponse::ok(data.unwrap(), "sasa")
}

