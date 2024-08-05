use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SseTarget {
    user_id: String,
    device_id: String,
    event_name: String,
    is_broadcast: bool,
    is_to_device: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SseBuilder<T> {
    target: SseTarget,
    pub data: Option<T>,
}

impl SseTarget {
    pub fn create() -> Self {
        SseTarget {
            user_id: "".to_string(),
            device_id: "".to_string(),
            event_name: "".to_string(),
            is_broadcast: false,
            is_to_device: false,
        }
    }

    pub fn broadcast(event_name: String) -> Self {
        SseTarget {
            user_id: "".to_string(),
            device_id: "".to_string(),
            event_name: event_name,
            is_broadcast: true,
            is_to_device: false,
        }
    }

    pub fn set_event_name(mut self, event_name: String) -> Self {
        self.event_name = event_name;
        self
    }

    pub fn set_user_id(mut self, user_id: String) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn set_device_id(mut self, device_id: String) -> Self {
        self.device_id = device_id;
        self.is_to_device = true;
        self
    }

    pub fn device_id(&self) -> &String {
        &self.device_id
    }
    pub fn even_name(&self) -> &String {
        &self.event_name
    }
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    pub fn is_broadcast(&self) -> bool {
        self.is_broadcast
    }

    pub fn is_to_device(&self) -> bool {
        self.is_to_device
    }
}


impl<T> SseBuilder<T> {
    pub fn new(
        target: SseTarget,
        data: T,
    ) -> SseBuilder<T> {
        SseBuilder {
            target,
            data: Some(data),
        }
    }


    pub fn get_target(&self) -> &SseTarget {
        &self.target
    }
}

impl<T> IntoResponse for SseBuilder<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
