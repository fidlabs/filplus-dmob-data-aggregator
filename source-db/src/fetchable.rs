use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Map,
    query_as, Error, Postgres,
};
use types::{CidSharing, ProviderDistribution, ReplicaDistribution};

pub trait Fetchable: Send + Sized + Unpin {
    const NAME: &'static str;

    fn query(
    ) -> Map<'static, Postgres, impl Send + FnMut(PgRow) -> Result<Self, Error>, PgArguments>;
}

impl Fetchable for ProviderDistribution {
    const NAME: &'static str = "ProviderDistribution";

    fn query(
    ) -> Map<'static, Postgres, impl Send + FnMut(PgRow) -> Result<Self, Error>, PgArguments> {
        // FIXME need proper query
        query_as!(
            Self,
            r#"
                select
                    'a' as "client!",
                    'b' as "provider!",
                    1 as "total_deal_size!",
                    1 as "unique_data_size!"
            "#
        )
    }
}

impl Fetchable for ReplicaDistribution {
    const NAME: &'static str = "ReplicaDistribution";

    fn query(
    ) -> Map<'static, Postgres, impl Send + FnMut(PgRow) -> Result<Self, Error>, PgArguments> {
        // FIXME need proper query
        query_as!(
            Self,
            "
                select
                    'a' as \"client!\",
                    1 as \"num_of_replicas!\",
                    1 as \"total_deal_size!\",
                    1 as \"unique_data_size!\"
            "
        )
    }
}

impl Fetchable for CidSharing {
    const NAME: &'static str = "CidSharing";

    fn query(
    ) -> Map<'static, Postgres, impl Send + FnMut(PgRow) -> Result<Self, Error>, PgArguments> {
        // FIXME need proper query
        query_as!(
            Self,
            "
                select
                    'a' as \"client!\",
                    'b' as \"other_client!\",
                    1 as \"unique_cid_count!\",
                    1 as \"total_deal_size!\"
            "
        )
    }
}
