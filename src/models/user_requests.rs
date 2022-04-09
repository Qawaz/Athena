use serde::Deserialize;

pub struct GetUserByIDReq {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
pub struct GetMultipleUsers {
    pub ids: Vec<i32>,
}
