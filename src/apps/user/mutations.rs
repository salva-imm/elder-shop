use crate::apps::base::query_characters;
use crate::apps::user::models::User;
use async_graphql::parser::types::Field;
use async_graphql::registry::Registry;
use async_graphql::{
    connection::{query, Connection, Edge},
    Context, ContextSelectionSet, Enum, Error, Interface, Object, OutputType, Positioned, Result,
    ServerResult, SimpleObject,
};
use std::borrow::Cow;

// #[Object]
// impl User {
//     async fn login(&self, username: String, password: String) -> String {
//         String::from("String")
//     }
// }
