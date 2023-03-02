use crate::apps::base::query_characters;
use crate::apps::shop::models::Shop;
use async_graphql::parser::types::Field;
use async_graphql::registry::Registry;
use async_graphql::{
    connection::{query, Connection, Edge},
    Context, ContextSelectionSet, Enum, Error, Interface, Object, OutputType, Positioned, Result,
    ServerResult, SimpleObject,
};
use std::borrow::Cow;

#[derive(Default)]
pub struct ShopResolvers;

#[Object]
impl ShopResolvers {
    async fn shops(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, Shop>> {
        let data = vec![
            Shop {
                id: 1,
                name: "Shop1".to_string(),
                domain: "https://shop.com".to_string(),
            },
            Shop {
                id: 2,
                name: "Shop2".to_string(),
                domain: "https://shop2.com".to_string(),
            },
            Shop {
                id: 3,
                name: "Shop3".to_string(),
                domain: "https://shop3.com".to_string(),
            },
        ];
        query_characters(after, before, first, last, data).await
    }
}
