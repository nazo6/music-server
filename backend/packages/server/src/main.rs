use std::{env, net::SocketAddr, str::FromStr};

use axum::Server;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use server_background::{BackgroundActor, BackgroundCommand, BackgroundEvent};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .pretty()
        .init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    tracing::debug!("database url {}", &db_url);
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let (bg_event_sender, bg_event_receiver) = async_channel::bounded::<BackgroundEvent>(1);
    let (bg_command_sender, bg_command_receiver) = mpsc::channel::<BackgroundCommand>(1);

    let mut bg_actor = BackgroundActor::new(bg_command_receiver, bg_event_sender);

    let app = api::init(bg_event_receiver, bg_command_sender);
    let addr = SocketAddr::from_str(&server_url()).unwrap();
    tracing::debug!("listening on {}", addr);

    tokio::select! {
        res = Server::bind(&addr).serve(app.into_make_service()) => {
            dbg!(res);
        },
        res = bg_actor.run() => {
            dbg!(res);
        }
    };

    Ok(())
}

fn server_url() -> String {
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    format!("{}:{}", host, port)
}
