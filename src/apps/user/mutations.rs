use crate::apps::base::query_characters;
use crate::apps::user::models::{Login, User};
use async_graphql::parser::types::Field;
use async_graphql::registry::Registry;
use async_graphql::{
    connection::{query, Connection, Edge},
    Context, ContextSelectionSet, Enum, Error, Interface, Object, OutputType, Positioned, Result,
    ServerResult, SimpleObject,
};
use std::borrow::Cow;

#[derive(Default)]
pub struct UserMutations;

#[Object]
impl UserMutations {
    async fn login(&self, username: String, password: String) -> Login {
        Login {
            token: String::from("String"),
        }
    }
}
