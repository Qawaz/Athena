use actix::Message;
use serde::{Deserialize, Serialize};

#[derive(Message, Debug, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct DeliveryReport {
    pub event: String,
    pub data: DeliveryReportContent,
}
#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryReportContent {
    pub sender: usize,
    pub ids: Vec<i32>,
}

impl DeliveryReportContent {
    pub fn set_sender_id_from_jwt(&mut self, sender_id: usize) {
        self.sender = sender_id
    }
}

#[derive(Message, Debug, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct DeliveryDeletedReport {
    pub event: String,
    pub data: DeliveryDeletedContent,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryDeletedContent {
    pub sender: usize,
    pub ids: Vec<i32>,
}

impl DeliveryDeletedContent {
    pub fn set_sender_id_from_jwt(&mut self, sender_id: usize) {
        self.sender = sender_id
    }
}
