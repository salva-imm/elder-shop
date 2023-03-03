use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Deserialize, Serialize, Default, Debug, Clone)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub action: String,
    pub created_at: String,
}
