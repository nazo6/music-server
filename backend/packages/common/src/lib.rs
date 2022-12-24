use std::env;

use once_cell::sync::Lazy;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub static DB_CONN: Lazy<DatabaseConnection> = Lazy::new(|| {
    let rt = Runtime::new().unwrap();

    // Spawn the root task
    rt.block_on(async {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        Database::connect(db_url)
            .await
            .expect("Database connection failed")
    })
});
