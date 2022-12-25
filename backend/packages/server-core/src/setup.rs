use common::get_db;
use entity::current::*;
use sea_orm::prelude::*;

use crate::{errors::Error, user::create_user_and_token};

/// Add admin user if not initialized and returns access token.
/// If least one user exists, do nothing and returns Ok(None)
pub async fn create_admin_user(username: &str, password: &str) -> Result<Option<String>, Error> {
    // if count of user table is zero, true
    let is_empty = user::Entity::find().count(get_db().await).await? == 0;

    if is_empty {
        let token = create_user_and_token(username, password, true).await?;
        Ok(Some(token))
    } else {
        Ok(None)
    }
}
