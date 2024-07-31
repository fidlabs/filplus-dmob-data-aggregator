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
