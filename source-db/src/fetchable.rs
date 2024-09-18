use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Map,
    query_as, Error, Postgres,
};
use types::{
    AggregatedClientDeals, AllocatorDistribution, CidSharing, ProviderDistribution, Providers,
    ReplicaDistribution,
};

pub trait Fetchable: Send + Sized + Unpin {
    const NAME: &'static str;

    fn query(
    ) -> Map<'static, Postgres, impl Send + FnMut(PgRow) -> Result<Self, Error>, PgArguments>;
}

impl Fetchable for ProviderDistribution {
    const NAME: &'static str = "ProviderDistribution";

    fn query(
    ) -> Map<'static, Postgres, impl Send + FnMut(PgRow) -> Result<Self, Error>, PgArguments> {
        query_as!(
            Self,
            r#"
             WITH miner_pieces AS (
                SELECT
                    'f0' || "clientId"  AS client,
                    'f0' || "providerId"  AS provider,
                    "pieceCid",
                    SUM("pieceSize") AS total_deal_size,
                    MIN("pieceSize") AS piece_size
                FROM  unified_verified_deal
                WHERE "termStart" > 0 and "sectorId" != '0'
                GROUP BY
                    client,
                    provider,
                    "pieceCid"
            ),
            miners AS (
                SELECT
                    client,
                    provider,
                    SUM(total_deal_size) AS total_deal_size,
                    SUM(piece_size)      AS unique_data_size
                FROM   miner_pieces
                GROUP  BY client, provider
            )
            SELECT
                client as "client!",
                provider as "provider!",
                total_deal_size::bigint as "total_deal_size!",
                unique_data_size::bigint as "unique_data_size!"
            FROM   miners
            ORDER  BY total_deal_size DESC
            "#
        )
    }
}

impl Fetchable for ReplicaDistribution {
    const NAME: &'static str = "ReplicaDistribution";

    fn query(
    ) -> Map<'static, Postgres, impl Send + FnMut(PgRow) -> Result<Self, Error>, PgArguments> {
        query_as!(
            Self,
            r#"
            WITH replicas AS (
                SELECT
                    'f0' || "clientId" as "clientId",
                    "pieceCid" AS piece_cid,
                    COUNT(DISTINCT "providerId") AS num_of_replicas,
                    SUM("pieceSize") AS total_deal_size,
                    MAX("pieceSize") AS piece_size
                FROM unified_verified_deal
                WHERE "termStart" > 0 and "sectorId" != '0'
                GROUP BY
                    "clientId",
                    piece_cid
            )
            SELECT
                "clientId" as "client!",
                num_of_replicas::INT as "num_of_replicas!",
                SUM(total_deal_size)::bigint AS "total_deal_size!",
                SUM(piece_size)::bigint AS "unique_data_size!"
            FROM replicas
            GROUP BY
                "clientId",
                num_of_replicas
            "#
        )
    }
}

impl Fetchable for CidSharing {
    const NAME: &'static str = "CidSharing";

    fn query(
    ) -> Map<'static, Postgres, impl Send + FnMut(PgRow) -> Result<Self, Error>, PgArguments> {
        query_as!(
            Self,
            r#"
            with cids as (
                    select distinct
                        "clientId",
                        "pieceCid"
                    from unified_verified_deal
            )
            SELECT 
                'f0' || cids."clientId" as "client!",
                'f0' || other_dc."clientId" as "other_client!",
                SUM(other_dc."pieceSize")::bigint AS "total_deal_size!",
                COUNT(DISTINCT other_dc."pieceCid")::INT AS "unique_cid_count!" 
            FROM 
                cids
            JOIN unified_verified_deal other_dc
                ON
                    cids."pieceCid" = other_dc."pieceCid"
                    and cids."clientId" != other_dc."clientId"
            GROUP BY 1, 2
            "#
        )
    }
}

impl Fetchable for AggregatedClientDeals {
    const NAME: &'static str = "AggregatedClientDeals";

    fn query(
    ) -> Map<'static, Postgres, impl Send + FnMut(PgRow) -> Result<Self, Error>, PgArguments> {
        query_as!(
            Self,
            r#"
            with aggregates as (
                select
                    'f0' || "clientId" as client,
                    "termStart" * 30 / 3600 as term_start,
                    sum("pieceSize") as total_deal_size
                from unified_verified_deal
                where "termStart" > 0 and "sectorId" != '0'
                group by 1, 2
            )
            select
                client as "client!",
                term_start * 3600 / 30 as "term_start_from!",
                (term_start+1) * 3600 / 30 - 1 as "term_start_to!",
                total_deal_size::bigint as "total_deal_size!"
            from aggregates
            "#
        )
    }
}

impl Fetchable for Providers {
    const NAME: &'static str = "Providers";

    fn query(
    ) -> Map<'static, Postgres, impl Send + FnMut(PgRow) -> Result<Self, Error>, PgArguments> {
        query_as!(
            Self,
            r#"
            select
                distinct on ("providerId")
                'f0' || "providerId" as "provider!",
                'f0' || "clientId" as "first_client!"
            from unified_verified_deal
            where "termStart" > 0 and "sectorId" != '0'
            order by
                "providerId",
                "termStart" asc
            "#
        )
    }
}

impl Fetchable for AllocatorDistribution {
    const NAME: &'static str = "AllocatorDistribution";

    fn query(
    ) -> Map<'static, Postgres, impl Send + FnMut(PgRow) -> Result<Self, Error>, PgArguments> {
        query_as!(
            Self,
            r#"
            select
                "verifierAddressId" as "allocator!",
                "addressId" as "client!",
                count(*) as "num_of_allocations!",
                sum(allowance)::bigint as "sum_of_allocations!"
            from verified_client_allowance
            where "addressId" != ''
            group by 1, 2;
            "#
        )
    }
}
