use crate::errors::Error;
use crate::user::get_user;
use bcrypt::{hash, DEFAULT_COST};
use common::DB_CONN;
use entity::current::*;
use sea_orm::{prelude::*, Set, TransactionTrait};
use uuid::Uuid;

pub async fn new_access_token(username: &str) -> Result<String, Error> {
    let token = Uuid::new_v4().to_string();
    let token_hash = hash(&token, DEFAULT_COST).map_err(|_| Error::GeneralError("".to_string()))?;

    let txn = (*DB_CONN).begin().await?;

    let user = get_user(username).await?;

    access_token::ActiveModel {
        token_hash: Set(token_hash),
        user_id: Set(user.id),
    }
    .insert(&txn)
    .await?;

    txn.commit().await?;

    Ok(token)
}
