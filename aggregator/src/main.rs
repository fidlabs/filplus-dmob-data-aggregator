pub use color_eyre::eyre::Result;
use color_eyre::eyre::WrapErr;
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

mod process;
mod spark;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug,sqlx=info"));

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(env_filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let source_db_url = std::env::var("SOURCE_DATABASE_URL")
        .context("Error reading env variable SOURCE_DATABASE_URL")?;
    let target_db_url = std::env::var("TARGET_DATABASE_URL")
        .context("Error reading env variable TARGET_DATABASE_URL")?;

    let source_db = source_db::SourceDatabase::connect(&source_db_url).await?;
    let dest_db = dest_db::DestDatabase::init(&target_db_url).await?;

    process::process(source_db, dest_db).await?;

    Ok(())
}
