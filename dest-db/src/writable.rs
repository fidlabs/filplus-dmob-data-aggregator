use sqlx::{postgres::PgArguments, query::Query, Postgres};
use types::{
    AggregatedClientDeals, AllocatorDistribution, CidSharing, ProviderDistribution,
    ProviderRetrievability, Providers, ReplicaDistribution,
};

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

impl Writable for AggregatedClientDeals {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        sqlx::query!(
                "
                    insert into aggregated_client_deals (client, term_start_from, term_start_to, total_deal_size)
                    values ($1, $2, $3, $4)
                ",
                self.client,
                self.term_start_from,
                self.term_start_to,
                self.total_deal_size
        )
    }

    fn truncate() -> Query<'static, Postgres, PgArguments> {
        sqlx::query!("truncate aggregated_client_deals")
    }
}

impl Writable for Providers {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        sqlx::query!(
            "
                    insert into providers (provider, first_client)
                    values ($1, $2)
                ",
            self.provider,
            self.first_client
        )
    }

    fn truncate() -> Query<'static, Postgres, PgArguments> {
        sqlx::query!("truncate providers")
    }
}

impl Writable for ProviderRetrievability {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        sqlx::query!(
            "
                    insert into provider_retrievability (provider, total, successful, success_rate)
                    values ($1, $2, $3, $4)
                ",
            self.provider,
            self.total,
            self.successful,
            self.success_rate
        )
    }

    fn truncate() -> Query<'static, Postgres, PgArguments> {
        sqlx::query!("truncate provider_retrievability")
    }
}

impl Writable for AllocatorDistribution {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        sqlx::query!(
            "
                    insert into allocator_distribution (allocator, client, num_of_allocations, sum_of_allocations)
                    values ($1, $2, $3, $4)
                ",
            self.allocator,
            self.client,
            self.num_of_allocations,
            self.sum_of_allocations
        )
    }

    fn truncate() -> Query<'static, Postgres, PgArguments> {
        sqlx::query!("truncate allocator_distribution")
    }
}
