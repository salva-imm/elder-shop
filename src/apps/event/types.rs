use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Deserialize, Serialize, Default, Debug, Clone)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(SimpleObject, Deserialize, Serialize, Default, Debug, Clone)]
pub struct OkOrNot {
    pub ok: bool,
}
