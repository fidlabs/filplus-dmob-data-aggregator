use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

pub struct ProviderDistribution {
    pub client: String,
    pub provider: String,
    pub total_deal_size: i64,
    pub unique_data_size: i64,
}

pub struct ReplicaDistribution {
    pub client: String,
    pub num_of_replicas: i32,
    pub total_deal_size: i64,
    pub unique_data_size: i64,
}

pub struct CidSharing {
    pub client: String,
    pub other_client: String,
    pub unique_cid_count: i32,
    pub total_deal_size: i64,
}

pub struct AggregatedClientDeals {
    pub client: String,
    pub term_start_from: i32,
    pub term_start_to: i32,
    pub total_deal_size: i64,
}

pub struct Providers {
    pub provider: String,
    pub first_client: String,
}

#[derive(Deserialize)]
pub struct ProviderRetrievability {
    #[serde(rename = "miner_id")]
    pub provider: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total: i64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub successful: i64,
    pub success_rate: f64,
}

pub struct AllocatorDistribution {
    pub allocator: String,
    pub client: String,
    pub num_of_allocations: i64,
    pub sum_of_allocations: i64,
}
