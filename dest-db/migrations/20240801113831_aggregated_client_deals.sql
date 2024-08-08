create table aggregated_client_deals (
	client text not null,
    term_start_from int not null,
    term_start_to int not null,
	total_deal_size bigint not null,
	primary key (client, term_start_from)
);
