use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Deserialize, Serialize, Default, Debug, Clone)]
pub struct Shop {
    pub id: i32,
    pub name: String,
    pub domain: String,
}
