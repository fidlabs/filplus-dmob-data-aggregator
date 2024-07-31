use crate::Result;
use dest_db::DestDatabase;
use source_db::SourceDatabase;
use tracing::info;

#[tracing::instrument(skip(source_db, dest_db))]
pub async fn process(source_db: SourceDatabase, dest_db: DestDatabase) -> Result<()> {
    process_provider_distributions(&source_db, &dest_db).await?;
    process_replica_distributions(&source_db, &dest_db).await?;
    process_cid_sharings(&source_db, &dest_db).await?;
    Ok(())
}

// FIXME deduplicate this with generics

#[tracing::instrument(skip(source_db, dest_db))]
pub async fn process_provider_distributions(
    source_db: &SourceDatabase,
    dest_db: &DestDatabase,
) -> Result<()> {
    info!("Fetching provider distributions");
    let data = source_db.fetch_provider_distributions().await?;

    info!("Writing provider distributions");
    dest_db
        .begin()
        .await?
        .truncate_provider_distributions()
        .await?
        .write_provider_distributions(data)
        .await?
        .commit()
        .await?;

    info!("Done");
    Ok(())
}

#[tracing::instrument(skip(source_db, dest_db))]
pub async fn process_replica_distributions(
    source_db: &SourceDatabase,
    dest_db: &DestDatabase,
) -> Result<()> {
    info!("Fetching replica_distributions");
    let data = source_db.fetch_replica_distributions().await?;

    info!("Writing replica distributions");
    dest_db
        .begin()
        .await?
        .truncate_replica_distributions()
        .await?
        .write_replica_distributions(data)
        .await?
        .commit()
        .await?;

    info!("Done");
    Ok(())
}

#[tracing::instrument(skip(source_db, dest_db))]
pub async fn process_cid_sharings(
    source_db: &SourceDatabase,
    dest_db: &DestDatabase,
) -> Result<()> {
    info!("Fetching cid sharings");
    let data = source_db.fetch_cid_sharings().await?;

    info!("Writing cid sharings");
    dest_db
        .begin()
        .await?
        .truncate_cid_sharings()
        .await?
        .write_cid_sharings(data)
        .await?
        .commit()
        .await?;

    info!("Done");
    Ok(())
}
