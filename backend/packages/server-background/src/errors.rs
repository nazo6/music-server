use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error: {0}")]
    GeneralError(String),
    #[error(transparent)]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error(transparent)]
    CoreError(#[from] server_core::errors::Error),
    #[error(transparent)]
    LoftyError(#[from] lofty::error::LoftyError),
}
