use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Deserialize, Serialize, Default, Debug, Clone)]
pub struct Order {
    pub id: i32,
    pub user: i32,
    pub email: Option<String>,
    pub billing_address: String,  // change to FK
    pub shipping_address: String, // change to FK
    pub shipping_method: String,  // change to FK
    pub currency: String,         // change to Enum
    pub country: String,          // change to Enum
    pub total: i64,               // change to decimal
    pub shipping_price: String,   // change to decimal
    pub discount_amount: String,  // change to decimal
}
