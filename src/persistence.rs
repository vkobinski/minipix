use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::time::interval;

use crate::AppState;

pub type PersistenceResult<T> = Result<T, sqlx::Error>;

pub struct PostgresRepository {
    pub pool: PgPool,
}

impl PostgresRepository {
    pub async fn connect(url: &str, pool_size: u32) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?;

        Ok(PostgresRepository { pool })
    }
}

pub async fn check_timeout(state: AppState) {
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));

        loop {
            interval.tick().await;

            state.check_timeout().await;
        }
    });
}
