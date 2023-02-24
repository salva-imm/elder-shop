use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};



// #[derive(SimpleObject)]
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub fullname: String,
    pub username: String,
    pub password: String,
    pub is_active: bool,
}

