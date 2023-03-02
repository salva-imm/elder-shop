use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Deserialize, Serialize, Default, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub fullname: String,
    pub username: String,
    pub password: String,
    pub is_active: bool,
}

#[derive(SimpleObject, Deserialize, Serialize, Default, Debug, Clone)]
pub struct Login {
    pub token: String,
}
