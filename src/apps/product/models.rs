use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Deserialize, Serialize, Default, Debug, Clone)]
pub struct Product {
    name: String,
    slug: String,
    category: i32, // FK
    rating: i32,
    tax: i32,
}
