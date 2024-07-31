use sqlx::postgres::{PgPool, PgPoolOptions};

mod fetchable;
pub use fetchable::Fetchable;

pub struct SourceDatabase {
    pool: PgPool,
}

impl SourceDatabase {
    pub async fn connect(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn fetch<T: Fetchable>(&self) -> Result<Vec<T>, sqlx::Error> {
        let data = T::query().fetch_all(&self.pool).await?;
        Ok(data)
    }
}
