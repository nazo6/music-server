use crate::AppState;

use super::{
    guard::{Role, RoleGuard},
    output_objects::Library,
};
use async_graphql::{Context, Object, Result};
use server_background::BackgroundCommand;
use server_core::{library::create_library, user::create_user};
use tokio::sync::oneshot;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn create_user(
        &self,
        _ctx: &Context<'_>,
        #[graphql(desc = "Name of user")] name: String,
        #[graphql(desc = "Password of user")] password: String,
        #[graphql(desc = "Is admin user")] is_admin: bool,
    ) -> Result<String> {
        create_user(&name, &password, is_admin).await?;
        Ok("success".to_string())
    }
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn create_library(
        &self,
        _ctx: &Context<'_>,
        #[graphql(desc = "Name of library")] name: String,
        #[graphql(desc = "Library path")] path: String,
    ) -> Result<Library> {
        let library = create_library(&name, &path).await?;
        Ok(Library {
            name: library.name,
            path: library.path,
            id: library.id,
        })
    }
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn start_scan(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Library id")] library_id: i32,
    ) -> Result<String> {
        let state = ctx.data::<AppState>().unwrap();
        let (responder, receiver) = oneshot::channel();
        state
            .background_command_sender
            .send(BackgroundCommand::StartScan {
                responder,
                library_id,
            })
            .await?;
        let response = receiver.await?;

        Ok(format!(
            "Started: {}, Message: {}",
            response.started, response.message
        ))
    }
}
