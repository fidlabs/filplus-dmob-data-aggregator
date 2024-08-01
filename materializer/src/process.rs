use crate::Result;
use dest_db::{DestDatabase, Writable};
use source_db::{Fetchable, SourceDatabase};
use tracing::{warn, info};
use types::{AggregatedClientDeals, CidSharing, ProviderDistribution, ReplicaDistribution};
use futures_util::future::TryFutureExt;

#[tracing::instrument(skip(source_db, dest_db))]
pub async fn process(source_db: SourceDatabase, dest_db: DestDatabase) -> Result<()> {
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
    let fetch = || source_db.fetch::<T>();
    let data = fetch()
        .or_else(|e| { 
            // we're reading from a read-only postgres replica. long running
            // queries can be cancelled if they're reading rows that primary db
            // wants to remove/update. for now, lets just retry. if this keeps
            // being a problem, increase max_standby_archive_delay and
            // max_standby_streaming_delay in replica config
            warn!(%e, "Error while fetching, retrying...");
            fetch()
        })
        .await?;


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
