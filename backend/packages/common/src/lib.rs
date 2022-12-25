use std::env;

use once_cell::sync::Lazy;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::sync::OnceCell;

static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db() -> &'static DatabaseConnection {
    DB_CONN
        .get_or_init(|| async {
            let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
            Database::connect(db_url)
                .await
                .expect("Database connection failed")
        })
        .await
}
