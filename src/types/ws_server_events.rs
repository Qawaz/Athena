use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestroyMessage {
    pub event: String,
    pub data: DestroyMessageContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestroyMessageContent {
    pub messages: Vec<MobileMessage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MobileMessage {
    pub local_id: Option<usize>,
    pub id: usize,
    pub sender: usize,
    pub receiver: usize,
    pub content: String,
    pub created_at: Option<String>,
}

// just changed schema to this because i have some problem with sending array with jsonobject in android!
// for sure later i should refactor
#[derive(Debug, Serialize)]
pub struct DestroyMessageDeliveryToReceiver {
    pub event: String,
    pub data: DestroyMessageDeliveryToReceiverContent,
}

#[derive(Debug, Serialize)]
pub struct DestroyMessageDeliveryToReceiverContent {
    pub message_ids: Vec<i32>,
}
