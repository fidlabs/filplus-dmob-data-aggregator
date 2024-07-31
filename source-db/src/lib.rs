use sqlx::postgres::{PgPool, PgPoolOptions};
use types::{CidSharing, ProviderDistribution, ReplicaDistribution};

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

    // FIXME deduplicate these with generics via some Fetchable trait on types
    pub async fn fetch_provider_distributions(
        &self,
    ) -> Result<Vec<ProviderDistribution>, sqlx::Error> {
        // perf improvement suggestion - implement streaming instead of collecting to Vec
        let data =
            sqlx::query_as_unchecked!( // FIXME make checked after query is provided
            ProviderDistribution,
            "select 'a' as client, 'a' as provider, 1 as total_deal_size, 1 as unique_data_size" // FIXME query here
        )
            .fetch_all(&self.pool)
            .await?;
        Ok(data)
    }

    pub async fn fetch_replica_distributions(
        &self,
    ) -> Result<Vec<ReplicaDistribution>, sqlx::Error> {
        let data = sqlx::query_as_unchecked!( // FIXME make checked after query is provided
            ReplicaDistribution,
            "select 'a' as client, 1 as num_of_replicas, 1 as total_deal_size, 1 as unique_data_size" // FIXME query here
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(data)
    }

    pub async fn fetch_cid_sharings(&self) -> Result<Vec<CidSharing>, sqlx::Error> {
        let data = sqlx::query_as_unchecked!( // FIXME make checked after query is provided
            CidSharing,
            "select 'a' as client, 'b' as other_client, 1 as unique_cid_count, 1 as total_deal_size" // FIXME query here
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(data)
    }
}
