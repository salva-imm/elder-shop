use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Deserialize, Serialize, Default, Debug, Clone)]
struct Invoice {
    order: i32, // FK
    number: i32,
    external_url: String,
    invoice_file: String,
}
