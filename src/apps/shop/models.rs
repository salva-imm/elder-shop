use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};



// #[derive(SimpleObject)]
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Shop {
    pub id: i32,
    pub name: String,
    pub domain: String,
}