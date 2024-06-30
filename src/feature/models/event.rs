use serde::{Serialize,Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct RegisterSse{
    pub user_id:String,
    pub channel_id:String
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SendToUserRequest{
    pub channel_id:String,
    pub user_id:String,
    pub event_name:String,
    pub message:String
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SendToUserChannel{
    pub channel_id:String,
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