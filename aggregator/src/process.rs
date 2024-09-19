use crate::{spark, Result};
use dest_db::{DestDatabase, Writable};
use source_db::{Fetchable, SourceDatabase};
use tracing::{info, warn};
use types::{
    AggregatedClientDeals, AllocatorDistribution, CidSharing, ProviderDistribution,
    ProviderRetrievability, Providers, ReplicaDistribution,
};

#[tracing::instrument(skip(source_db, dest_db))]
pub async fn process(source_db: SourceDatabase, dest_db: DestDatabase) -> Result<()> {
    process_view::<Providers>(&source_db, &dest_db).await?;
    process_view::<ProviderDistribution>(&source_db, &dest_db).await?;
    process_view::<ReplicaDistribution>(&source_db, &dest_db).await?;
    process_view::<CidSharing>(&source_db, &dest_db).await?;
    process_view::<AggregatedClientDeals>(&source_db, &dest_db).await?;
    process_view::<AllocatorDistribution>(&source_db, &dest_db).await?;
    process_retrievability(&dest_db).await?;
    Ok(())
}

#[tracing::instrument(skip(source_db, dest_db), fields(view=T::NAME))]
pub async fn process_view<T: Fetchable + Writable>(
    source_db: &SourceDatabase,
    dest_db: &DestDatabase,
) -> Result<()> {
    info!("Fetching");
    let data = source_db.fetch::<T>().await;
    let data = if data.is_err() {
        warn!("Fetch failed, retrying...");
        source_db.fetch::<T>().await?
    } else {
        data?
    };

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

#[tracing::instrument(skip_all)]
pub async fn process_retrievability(dest_db: &DestDatabase) -> Result<()> {
    info!("Fetching");
    let data = spark::fetch_retrievability_data().await?;

    info!("Writing {} rows", data.len());
    dest_db
        .begin()
        .await?
        .truncate::<ProviderRetrievability>()
        .await?
        .insert::<ProviderRetrievability>(data)
        .await?
        .commit()
        .await?;

    info!("Done");
    Ok(())
}
