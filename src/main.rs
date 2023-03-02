#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use actix_redis::RedisActor;
use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, MergedObject, Object, OutputType, Schema,
};
mod apps;
// use crate::apps::base::QueryRoot;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

// #[derive(MergedObject, Default)]
// struct Query(UserQuery, MovieQuery);

use crate::apps::user::models::User;

#[derive(Default)]
struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self) -> Vec<User>
    where
        User: OutputType + Clone,
    {
        vec![User {
            id: 1,
            fullname: "John Doe".to_string(),
            username: "johndoe".to_string(),
            password: "123456".to_string(),
            is_active: true,
        }]
    }
}

#[derive(Default)]
struct MovieQuery;

#[Object]
impl MovieQuery {
    async fn movies(&self) -> Vec<User>
    where
        User: OutputType + Clone,
    {
        vec![User {
            id: 1,
            fullname: "John Doe".to_string(),
            username: "johndoe".to_string(),
            password: "123456".to_string(),
            is_active: true,
        }]
    }
}

#[derive(MergedObject, Default)]
struct Query(UserQuery, MovieQuery);

type ShopSchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn index(schema: web::Data<ShopSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = RedisActor::start("localhost:6379");
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(addr.clone())
        .finish();

    // .data(addr.clone())

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
// #[derive(Debug)]
// struct User {
//     id: i32,
//     name: String,
// }
// fn get_something() -> Vec<User> {
//     vec![User {
//         id: 1,
//         name: "Hell".to_string(),
//     }]
// }
// fn main() {
//     println!("{:#?}", get_something());
// }
