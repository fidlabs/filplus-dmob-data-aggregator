use sqlx::{postgres::PgArguments, query::Query, Postgres};
use types::{CidSharing, ProviderDistribution, ReplicaDistribution};

pub trait Writable: Send + Sized + Unpin {
    fn insert(&self) -> Query<'static, Postgres, PgArguments>;
    fn truncate() -> Query<'static, Postgres, PgArguments>;
}

impl Writable for ProviderDistribution {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        sqlx::query!(
                "
                    insert into provider_distribution (client, provider, total_deal_size, unique_data_size)
                    values ($1, $2, $3, $4)
                ",
                self.client,
                self.provider,
                self.total_deal_size,
                self.unique_data_size
        )
    }

    fn truncate() -> Query<'static, Postgres, PgArguments> {
        sqlx::query!("truncate provider_distribution")
    }
}

impl Writable for ReplicaDistribution {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        sqlx::query!(
                "
                    insert into replica_distribution (client, num_of_replicas, total_deal_size, unique_data_size)
                    values ($1, $2, $3, $4)
                ",
                self.client,
                self.num_of_replicas,
                self.total_deal_size,
                self.unique_data_size
        )
    }

    fn truncate() -> Query<'static, Postgres, PgArguments> {
        sqlx::query!("truncate replica_distribution")
    }
}

impl Writable for CidSharing {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        sqlx::query!(
                "
                    insert into cid_sharing (client, other_client, unique_cid_count, total_deal_size)
                    values ($1, $2, $3, $4)
                ",
                self.client,
                self.other_client,
                self.unique_cid_count,
                self.total_deal_size
        )
    }

    fn truncate() -> Query<'static, Postgres, PgArguments> {
        sqlx::query!("truncate cid_sharing")
    }
}
