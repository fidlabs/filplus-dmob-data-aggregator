use crate::Result;
use dest_db::{DestDatabase, Writable};
use source_db::{Fetchable, SourceDatabase};
use tracing::info;
use types::{CidSharing, ProviderDistribution, ReplicaDistribution};

#[tracing::instrument(skip(source_db, dest_db))]
pub async fn process(source_db: SourceDatabase, dest_db: DestDatabase) -> Result<()> {
    process_view::<ProviderDistribution>(&source_db, &dest_db).await?;
    process_view::<ReplicaDistribution>(&source_db, &dest_db).await?;
    process_view::<CidSharing>(&source_db, &dest_db).await?;
    Ok(())
}

// FIXME instrumentation lacks info about actual type used here
#[tracing::instrument(skip(source_db, dest_db))]
pub async fn process_view<T: Fetchable + Writable>(
    source_db: &SourceDatabase,
    dest_db: &DestDatabase,
) -> Result<()> {
    info!("Fetching");
    let data = source_db.fetch::<T>().await?;

    info!("Writing");
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