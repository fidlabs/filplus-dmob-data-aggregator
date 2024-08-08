use std::time::Duration;

use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use sqlx::ConnectOptions;

mod fetchable;
pub use fetchable::Fetchable;

pub struct SourceDatabase {
    pool: PgPool,
}

impl SourceDatabase {
    pub async fn connect(db_url: &str) -> Result<Self, sqlx::Error> {
        let options = db_url
            .parse::<PgConnectOptions>()?
            .log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(5 * 60));

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        Ok(Self { pool })
    }

    pub async fn fetch<T: Fetchable>(&self) -> Result<Vec<T>, sqlx::Error> {
        let data = T::query().fetch_all(&self.pool).await?;
        Ok(data)
    }
}
