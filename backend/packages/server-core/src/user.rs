use crate::errors::Error;
use bcrypt::{hash, verify, DEFAULT_COST};
use entity::current::*;
use sea_orm::{prelude::*, ConnectionTrait, Set};

pub async fn add_user(
    username: &str,
    password: &str,
    is_admin: bool,
    conn: &impl ConnectionTrait,
) -> Result<(), Error> {
    let password_hash = hash(password, DEFAULT_COST)?;

    user::ActiveModel {
        name: Set(username.to_string()),
        password_hash: Set(password_hash.to_string()),
        is_admin: Set(is_admin),
        ..Default::default()
    }
    .insert(conn)
    .await?;

    Ok(())
}

pub async fn get_user_if_authed(
    username: &str,
    password: &str,
    conn: &impl ConnectionTrait,
) -> Result<Option<user::Model>, Error> {
    let user = user::Entity::find()
        .filter(user::Column::Name.eq(username))
        .one(conn)
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

pub async fn get_user(username: &str, conn: &impl ConnectionTrait) -> Result<user::Model, Error> {
    let user = user::Entity::find()
        .filter(user::Column::Name.eq(username))
        .one(conn)
        .await;

    match user {
        Ok(Some(user)) => Ok(user),
        _ => Err(Error::GeneralError("User not found".to_string())),
    }
}
