use crate::{access_token::create_token, errors::Error};
use bcrypt::{hash, verify, DEFAULT_COST};
use entity::current::*;
use sea_orm::{prelude::*, Set, TransactionTrait};

use common::get_db;
use tracing::instrument;

#[instrument]
pub async fn create_user(
    username: &str,
    password: &str,
    is_admin: bool,
) -> Result<user::Model, Error> {
    let password_hash = hash(password, DEFAULT_COST)?;

    let user = user::ActiveModel {
        name: Set(username.to_string()),
        password_hash: Set(password_hash.to_string()),
        is_admin: Set(is_admin),
        ..Default::default()
    }
    .insert(get_db().await)
    .await?;

    Ok(user)
}

#[instrument]
pub async fn create_user_and_token(
    username: &str,
    password: &str,
    is_admin: bool,
) -> Result<String, Error> {
    let txn = get_db().await.begin().await?;

    let user = create_user(username, password, is_admin).await?;

    let token = create_token(user.id).await?;

    txn.commit().await?;

    Ok(token)
}

#[instrument]
pub async fn get_user_if_authed(
    username: &str,
    password: &str,
) -> Result<Option<user::Model>, Error> {
    let user = user::Entity::find()
        .filter(user::Column::Name.eq(username))
        .one(get_db().await)
        .await;

    match user {
        Ok(Some(user)) => {
            let verified = verify(password, &user.password_hash)?;
            if verified {
                Ok(Some(user))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

#[instrument]
pub async fn get_user_from_id(username: &str) -> Result<user::Model, Error> {
    let user = user::Entity::find()
        .filter(user::Column::Name.eq(username))
        .one(get_db().await)
        .await;

    match user {
        Ok(Some(user)) => Ok(user),
        _ => Err(Error::GeneralError("User not found".to_string())),
    }
}

#[instrument]
pub async fn get_user(user_id: i32) -> Result<user::Model, Error> {
    let user = user::Entity::find_by_id(user_id).one(get_db().await).await;

    match user {
        Ok(Some(user)) => Ok(user),
        _ => Err(Error::GeneralError("User not found".to_string())),
    }
}

#[instrument]
pub async fn get_users() -> Result<Vec<user::Model>, Error> {
    let users = user::Entity::find().all(get_db().await).await;

    match users {
        Ok(users) => Ok(users),
        _ => Err(Error::GeneralError("User not found".to_string())),
    }
}
