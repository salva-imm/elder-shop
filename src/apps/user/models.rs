use std::borrow::Cow;
use async_graphql::{connection::{query, Connection, Edge}, Context, ContextSelectionSet, Enum, Error, Interface, Object, OutputType, Positioned, Result, ServerResult, SimpleObject};
use async_graphql::parser::types::Field;
use async_graphql::registry::Registry;
use serde::{Deserialize, Serialize};

use crate::apps::base::{QueryRoot, query_characters};

#[derive(SimpleObject)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    id: i32,
    fullname: String,
    username: String,
    password: String,
    is_active: bool,
}

#[Object]
impl QueryRoot {
    async fn users(&self,
                   ctx: &Context<'_>,
                   after: Option<String>,
                    before: Option<String>,
                    first: Option<i32>,
                    last: Option<i32>
                   ) -> Result<Connection<usize, User>> {
        let data = vec![
            User {
                id: 1,
                fullname: "John Doe".to_string(),
                username: "johndoe".to_string(),
                password: "123456".to_string(),
                is_active: true,
            },
            User {
                id: 2,
                fullname: "Jane Doe".to_string(),
                username: "janedoe".to_string(),
                password: "123456".to_string(),
                is_active: true,
            },
            User {
                id: 3,
                fullname: "John Smith".to_string(),
                username: "johnsmith".to_string(),
                password: "123456".to_string(),
                is_active: true,
            },
            User {
                id: 4,
                fullname: "Jane Smith".to_string(),
                username: "janesmith".to_string(),
                password: "123456".to_string(),
                is_active: true,
            },
        ];
        query_characters(after, before, first, last, data).await
    }
}
