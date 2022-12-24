use crate::errors::Error;
use bcrypt::{hash, verify, DEFAULT_COST};
use entity::current::*;
use sea_orm::{prelude::*, Set};

use common::DB_CONN;

pub async fn add_user(username: &str, password: &str, is_admin: bool) -> Result<(), Error> {
    let password_hash = hash(password, DEFAULT_COST)?;

    user::ActiveModel {
        name: Set(username.to_string()),
        password_hash: Set(password_hash.to_string()),
        is_admin: Set(is_admin),
        ..Default::default()
    }
    .insert(&*DB_CONN)
    .await?;

    Ok(())
}

pub async fn get_user_if_authed(
    username: &str,
    password: &str,
) -> Result<Option<user::Model>, Error> {
    let user = user::Entity::find()
        .filter(user::Column::Name.eq(username))
        .one(&*DB_CONN)
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

pub async fn get_user(username: &str) -> Result<user::Model, Error> {
    let user = user::Entity::find()
        .filter(user::Column::Name.eq(username))
        .one(&*DB_CONN)
        .await;

    match user {
        Ok(Some(user)) => Ok(user),
        _ => Err(Error::GeneralError("User not found".to_string())),
    }
}
