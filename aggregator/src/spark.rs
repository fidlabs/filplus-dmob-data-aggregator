use crate::Result;
use chrono::{Days, Utc};
use types::ProviderRetrievability;

#[tracing::instrument]
pub async fn fetch_retrievability_data() -> Result<Vec<ProviderRetrievability>> {
    let today = Utc::now().date_naive();
    let to = today.format("%Y-%m-%d");
    let from = today
        .checked_sub_days(Days::new(1))
        .expect("Sane local clock configuration will not overflow this")
        .format("%Y-%m-%d");

    let data = reqwest::get(format!(
        "https://stats.filspark.com/miners/retrieval-success-rate/summary?from={from}&to={to}"
    ))
    .await?
    .json::<Vec<ProviderRetrievability>>()
    .await?;

    Ok(data)
}
