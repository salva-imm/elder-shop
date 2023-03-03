use crate::apps::base::query_characters;
use crate::apps::user::models::User;
use crate::apps::user::types::{LoginResponse, OkOrNot};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
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
    async fn ping(&self) -> Result<OkOrNot> {
        // Err(Error::from("That's how it ends"))
        Ok(OkOrNot { ok: true })
    }
    async fn register(&self, username: String, password: String) -> Result<OkOrNot> {
        let password = password.into_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(&password, &salt)?.to_string();
        println!("{}", password_hash);
        let parsed_hash = PasswordHash::new(&password_hash)?;
        let res = Argon2::default()
            .verify_password(&password, &parsed_hash)
            .is_ok();
        // Err(Error::from("That's how it ends"))
        Ok(OkOrNot { ok: res })
    }
    async fn login(&self, username: String, password: String) -> Result<LoginResponse> {
        let byte_password = password.into_bytes();
        let hashed_password = String::from("$argon2id$v=19$m=4096,t=3,p=1$C9vDMEBXZXHmEWm3IdDIAw$fbQ5V0iDCNqbY48QoCqpecd4oiDXalUlBE5OXSUqwPw");
        let parsed_hash = PasswordHash::new(&hashed_password)?;
        let res = Argon2::default()
            .verify_password(&byte_password, &parsed_hash)
            .is_ok();
        println!("{}", res);
        Ok(LoginResponse {
            token: String::from("String"),
        })
    }
}
