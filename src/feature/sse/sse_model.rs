use serde::{Serialize,Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct RegisterSse{
    pub device_id:String,
    pub user_id:String
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SendToUserRequest{
    pub user_id:String,
    pub device_id:String,
    pub event_name:String,
    pub message:String
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SendToUserChannel{
    pub user_id:String,
    pub event_name:String,
    pub message:String
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SendBroadcastRequest{
    pub event_name:String,
    pub message:String
}


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SendMessageResponse{
    pub message:String
}