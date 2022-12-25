use crate::{errors::Error, user::get_user};
use base64ct::{Base64, Encoding};
use common::get_db;
use entity::current::*;
use sea_orm::{prelude::*, Set};
use sha2::{Digest, Sha256};
use tracing::instrument;
use uuid::Uuid;

fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let hash = hasher.finalize();
    Base64::encode_string(&hash)
}

#[instrument]
pub async fn create_token(user_id: i32) -> Result<String, Error> {
    let token = Uuid::new_v4().to_string();
    let token_hash = hash_sha256(&token);

    let user = get_user(user_id).await?;

    access_token::ActiveModel {
        token_hash: Set(token_hash),
        user_id: Set(user.id),
    }
    .insert(get_db().await)
    .await?;

    Ok(token)
}

#[instrument]
pub async fn revoke_token(token: &str) -> Result<(), Error> {
    let token_hash = hash_sha256(token);

    let res = access_token::Entity::delete_by_id(token_hash)
        .exec(get_db().await)
        .await?;

    if res.rows_affected == 1 {
        Ok(())
    } else {
        Err(Error::GeneralError("Token not found.".to_string()))
    }
}

#[instrument]
pub async fn validate_token(token: &str) -> Result<Option<user::Model>, Error> {
    let token_hash = hash_sha256(token);
    let user = access_token::Entity::find()
        .find_also_related(user::Entity)
        .filter(access_token::Column::TokenHash.eq(token_hash))
        .one(get_db().await)
        .await?;

    if let Some(user) = user {
        Ok(Some(user.1.expect(
            "User referenced in access token not found. This should not happen.",
        )))
    } else {
        Ok(None)
    }
}
