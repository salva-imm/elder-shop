use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Shop {
    pub id: i32,
    pub name: String,
    pub domain: String,
}
