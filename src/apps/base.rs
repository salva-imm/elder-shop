use async_graphql::{
    connection::{query, Connection, Edge},
    Context, Enum, Error, Interface, MergedObject, Object, OutputType, Result, SimpleObject,
};

use crate::apps::shop::resolvers::ShopResolvers;
use crate::apps::user::mutations::UserMutations;
use crate::apps::user::resolvers::UserResolvers;

pub async fn query_characters<'a, T>(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    characters: Vec<T>,
) -> Result<Connection<usize, T>>
where
    T: OutputType + Clone,
{
    query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let mut start = 0usize;
            let mut end = characters.len();

            if let Some(after) = after {
                if after >= characters.len() {
                    return Ok(Connection::new(false, false));
                }
                start = after + 1;
            }

            if let Some(before) = before {
                if before == 0 {
                    return Ok(Connection::new(false, false));
                }
                end = before;
            }

            let mut slice = &characters[start..end];

            if let Some(first) = first {
                slice = &slice[..first.min(slice.len())];
                end -= first.min(slice.len());
            } else if let Some(last) = last {
                slice = &slice[slice.len() - last.min(slice.len())..];
                start = end - last.min(slice.len());
            }

            let mut connection = Connection::new(start > 0, end < characters.len());
            connection.edges.extend(
                slice
                    .to_vec()
                    .iter()
                    .enumerate()
                    .map(move |(idx, item)| Edge::new(start + idx, item.clone())),
            );
            Ok::<_, Error>(connection)
        },
    )
    .await
}

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserResolvers, ShopResolvers);

#[derive(MergedObject, Default)]
pub struct MutationRoot(UserMutations);
