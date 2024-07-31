-- Add migration script here

create table provider_distribution (
	client text not null,
	provider text not null,
	total_deal_size bigint not null,
	unique_data_size bigint not null,
	primary key(client, provider)
);

create table replica_distribution (
	client text not null,
	num_of_replicas int not null,
	total_deal_size bigint not null,
	unique_data_size bigint not null,
	primary key (client, num_of_replicas)
);

create table cid_sharing (
	client text not null,
	other_client text not null,
	unique_cid_count int not null,
	total_deal_size bigint not null,
	primary key (client, other_client)
);
