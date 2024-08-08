use sqlx::postgres::{PgPool, PgPoolOptions};

mod writable;
pub use writable::Writable;

pub struct DestDatabase {
    pool: PgPool,
}

pub struct Transaction {
    tx: sqlx::Transaction<'static, sqlx::Postgres>,
}

impl DestDatabase {
    pub async fn init(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        sqlx::migrate!().run(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn begin(&self) -> Result<Transaction, sqlx::Error> {
        let tx = self.pool.begin().await?;
        Ok(Transaction { tx })
    }
}

impl Transaction {
    pub async fn truncate<T: Writable>(mut self) -> Result<Self, sqlx::Error> {
        T::truncate().execute(&mut *self.tx).await?;
        Ok(self)
    }

    pub async fn insert<T: Writable>(mut self, data: Vec<T>) -> Result<Self, sqlx::Error> {
        // perf note - optimize this with some batching
        for row in data {
            row.insert().execute(&mut *self.tx).await?;
        }
        Ok(self)
    }

    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.tx.commit().await
    }
}
