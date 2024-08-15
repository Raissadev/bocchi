use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;

pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new() -> Database {
        let database_url = Self::configuration();
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Database connection failed");

        Database { pool }
    }

    fn configuration() -> String {
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://database.db".to_string())
    }
}
