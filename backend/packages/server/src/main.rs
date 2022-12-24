use std::{env, net::SocketAddr, str::FromStr};

use axum::{Router, Server};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    tracing::debug!("database url {}", &db_url);
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let app = api::init();

    let addr = SocketAddr::from_str(&server_url).unwrap();
    tracing::debug!("listening on {}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
