use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;
use tokio::sync::OnceCell;

pub struct DatabaseManager {
    pub connection: DatabaseConnection,
}

impl DatabaseManager {
    pub async fn new(url: String) -> Result<Self, DbErr> {
        let mut opt = ConnectOptions::new(url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);

        let connection = Database::connect(opt).await?;

        Ok(Self { connection })
    }

    pub async fn global() -> &'static Self {
        static INSTANCE: OnceCell<DatabaseManager> = OnceCell::const_new();

        INSTANCE
            .get_or_init(|| async {
                let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
                DatabaseManager::new(database_url)
                    .await
                    .expect("Failed to create database connection.")
            })
            .await
    }

    pub async fn ping(&self) -> bool {
        match self.connection.ping().await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("Database ping failed: {}.", e);
                false
            }
        }
    }

    pub async fn close(self) {
        drop(self);
        tracing::info!("Database connection closed.");
    }

    pub fn get_connection(&self) -> DatabaseConnection {
        self.connection.clone()
    }
}
