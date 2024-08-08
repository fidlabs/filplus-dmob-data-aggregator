use crate::Result;
use dest_db::{DestDatabase, Writable};
use futures_util::future::TryFutureExt;
use source_db::{Fetchable, SourceDatabase};
use tracing::{info, warn};
use types::{
    AggregatedClientDeals, CidSharing, ProviderDistribution, Providers, ReplicaDistribution,
};

#[tracing::instrument(skip(source_db, dest_db))]
pub async fn process(source_db: SourceDatabase, dest_db: DestDatabase) -> Result<()> {
    process_view::<Providers>(&source_db, &dest_db).await?;
    process_view::<ProviderDistribution>(&source_db, &dest_db).await?;
    process_view::<ReplicaDistribution>(&source_db, &dest_db).await?;
    process_view::<CidSharing>(&source_db, &dest_db).await?;
    process_view::<AggregatedClientDeals>(&source_db, &dest_db).await?;
    Ok(())
}

#[tracing::instrument(skip(source_db, dest_db), fields(view=T::NAME))]
pub async fn process_view<T: Fetchable + Writable>(
    source_db: &SourceDatabase,
    dest_db: &DestDatabase,
) -> Result<()> {
    info!("Fetching");
    let data = source_db.fetch::<T>().await?;

    info!("Writing {} rows", data.len());
    dest_db
        .begin()
        .await?
        .truncate::<T>()
        .await?
        .insert::<T>(data)
        .await?
        .commit()
        .await?;

    info!("Done");
    Ok(())
}
