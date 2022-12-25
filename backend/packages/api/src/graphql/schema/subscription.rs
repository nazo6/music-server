use async_graphql::*;
use futures::Stream;
use server_background::BackgroundEvent;

use crate::AppState;

use super::{
    guard::{Role, RoleGuard},
    output_objects::ScanStatus,
};

#[derive(Debug)]
pub struct Subscription;

#[Subscription]
impl Subscription {
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn scan_status(&self, ctx: &Context<'_>) -> impl Stream<Item = ScanStatus> {
        let state = ctx.data::<AppState>().unwrap();
        dbg!(&state);
        let reciever = state.background_event_receiver.clone();

        async_stream::stream! {
            while let Ok(item) = reciever.recv().await {
                match item {
                    BackgroundEvent::UpdateScan{scanning, count} => {
                        let status = ScanStatus {
                            scanning,
                            proceed_count: count,
                        };
                        yield status;
                    }
                    _ => {}
                }
            }
        }
    }
}
