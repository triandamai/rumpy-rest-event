use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug,Clone,Serialize,Deserialize,Validate)]
pub struct RegisterSse{
    #[validate(length(min = 1))]
    pub device_id:String,
    #[validate(length(min = 1))]
    pub user_id:String
}

#[derive(Debug,Clone,Serialize,Deserialize,Validate)]
pub struct SendToUserRequest{
    #[validate(length(min = 1))]
    pub user_id:String,
    #[validate(length(min = 1))]
    pub device_id:String,
    #[validate(length(min = 1))]
    pub event_name:String,
    #[validate(length(min = 1))]
    pub message:String
}

#[derive(Debug,Clone,Serialize,Deserialize,Validate)]
pub struct SendToUserChannel{
    #[validate(length(min = 1))]
    pub user_id:String,
    #[validate(length(min = 1))]
    pub event_name:String,
    #[validate(length(min = 1))]
    pub message:String
}

#[derive(Debug,Clone,Serialize,Deserialize,Validate)]
pub struct SendBroadcastRequest{
    #[validate(length(min = 1))]
    pub event_name:String,
    #[validate(length(min = 1))]
    pub message:String
}


#[derive(Debug,Clone,Serialize,Deserialize,Validate)]
pub struct SendMessageResponse{
    #[validate(length(min = 1))]
    pub message:String
}


#[derive(Debug,Clone,Serialize,Deserialize,Validate)]
pub struct SubscribeToTopicRequest{
    #[validate(length(min = 1))]
    pub user_id:String,
    #[validate(length(min = 1))]
    pub topic:String
}
