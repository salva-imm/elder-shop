use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Deserialize, Serialize, Default, Debug, Clone)]
pub struct Sale {
    pub id: i32,
    pub user: i32,
    pub name: String,
    pub discount_type: String, // change to Enum
    pub products: String,      // change to FK
}
