use sqlx::postgres::{PgPool, PgPoolOptions};
use types::{CidSharing, ProviderDistribution, ReplicaDistribution};

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
    // FIXME deduplicate this with generics via some Writable trait on types
    pub async fn truncate_provider_distributions(mut self) -> Result<Self, sqlx::Error> {
        sqlx::query!("truncate provider_distribution")
            .execute(&mut *self.tx)
            .await?;
        Ok(self)
    }

    pub async fn write_provider_distributions(
        mut self,
        data: Vec<ProviderDistribution>,
    ) -> Result<Self, sqlx::Error> {
        // perf improvement suggestion - does sqlx::query! macro prepare a statement once or each time we execute it?
        // perf improvement suggestion - batch multiple rows per query execution
        for row in data {
            sqlx::query!(
                "
                    insert into provider_distribution (client, provider, total_deal_size, unique_data_size)
                    values ($1, $2, $3, $4)
                ",
                row.client,
                row.provider,
                row.total_deal_size,
                row.unique_data_size
            )
            .execute(&mut *self.tx)
            .await?;
        }
        Ok(self)
    }

    pub async fn truncate_replica_distributions(mut self) -> Result<Self, sqlx::Error> {
        sqlx::query!("truncate replica_distributions")
            .execute(&mut *self.tx)
            .await?;
        Ok(self)
    }

    pub async fn write_replica_distributions(
        mut self,
        data: Vec<ReplicaDistribution>,
    ) -> Result<Self, sqlx::Error> {
        for row in data {
            sqlx::query!(
                "
                    insert into replica_distribution (client, num_of_replicas, total_deal_size, unique_data_size)
                    values ($1, $2, $3, $4)
                ",
                row.client,
                row.num_of_replicas,
                row.total_deal_size,
                row.unique_data_size
            )
            .execute(&mut *self.tx)
            .await?;
        }
        Ok(self)
    }

    pub async fn truncate_cid_sharings(mut self) -> Result<Self, sqlx::Error> {
        sqlx::query!("truncate cid_sharings")
            .execute(&mut *self.tx)
            .await?;
        Ok(self)
    }

    pub async fn write_cid_sharings(mut self, data: Vec<CidSharing>) -> Result<Self, sqlx::Error> {
        for row in data {
            sqlx::query!(
                "
                    insert into cid_sharing (client, other_client, unique_cid_count, total_deal_size)
                    values ($1, $2, $3, $4)
                ",
                row.client,
                row.other_client,
                row.unique_cid_count,
                row.total_deal_size
            )
            .execute(&mut *self.tx)
            .await?;
        }
        Ok(self)
    }

    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.tx.commit().await
    }
}
