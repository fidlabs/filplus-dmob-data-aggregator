# What is it?

This tool reads data from DMOB database, aggregates it and writes the result to another database. It's used to provide data for [CID Checker](https://github.com/fidlabs/filplus-checker) and [Allocator Compliance Checker](https://github.com/fidlabs/allocator-compliance-checker).

# How to use it?

To run it once:

```
env SOURCE_DATABASE_URL=postgres://user:pass@host:port/name # URL to DMOB DB replica (can be read-only)
env TARGET_DATABASE_URL=postgres://user:pass@host:port/name # URL to a writable DB where aggregates will be saved
cargo run --release
```

But in production use systemd to run it periodically:
```
[Unit]
Description=Run filplus-dmob-data-aggregator at 1h intervals
After=network.target

[Service]
Environment="SOURCE_DATABASE_URL=postgres://user:pass@host:port/name"
Environment="TARGET_DATABASE_URL=postgres://user:pass@host:port/name"
ExecStart=/absolute/path/to/filplus-dmob-data-aggregator/target/release/aggregator
RestartSec=1h
Restart=always

[Install]
WantedBy=multi-user.target
```

# What data it produces?

See [read queries](./source-db/src/fetchable.rs) and [write queries](./dest-db/src/writable.rs) for up-to-date info.


## Providers table

Contains list of known Storage Providers together with the first verified client that we've seen using them:

```
postgres=# \d providers;
               Table "public.providers"
    Column    | Type | Collation | Nullable | Default 
--------------+------+-----------+----------+---------
 provider     | text |           | not null | 
 first_client | text |           | not null | 
Indexes:
    "providers_pkey" PRIMARY KEY, btree (provider)

postgres=# select * from providers limit 2;
 provider  | first_client 
-----------+--------------
 f01016847 | f02049082
 f01019009 | f02049082
(2 rows)
```

## Provider Distribution table

Each row contains:
* client id
* provider id
* sum of sizes of all deals between this client & provider
* total size of unique data in deals between this client & provider

```
postgres=# \d provider_distribution;
            Table "public.provider_distribution"
      Column      |  Type  | Collation | Nullable | Default 
------------------+--------+-----------+----------+---------
 client           | text   |           | not null | 
 provider         | text   |           | not null | 
 total_deal_size  | bigint |           | not null | 
 unique_data_size | bigint |           | not null | 
Indexes:
    "provider_distribution_pkey" PRIMARY KEY, btree (client, provider)

postgres=# select * from provider_distribution limit 2;
  client   | provider  | total_deal_size  | unique_data_size 
-----------+-----------+------------------+------------------
 f01187995 | f03144188 | 4278474621583360 | 4278474621583360
 f03096139 | f03079766 | 2620754684280832 | 2620754684280832
(2 rows)
```

## Replica Distribution table

Each row contains:
* client id
* number of replicas
* sum of sizes of all deals this client made with this number of replicas
* total size of unique data in deals this client made with this number of replicas


```
postgres=# \d replica_distribution;
             Table "public.replica_distribution"
      Column      |  Type   | Collation | Nullable | Default 
------------------+---------+-----------+----------+---------
 client           | text    |           | not null | 
 num_of_replicas  | integer |           | not null | 
 total_deal_size  | bigint  |           | not null | 
 unique_data_size | bigint  |           | not null | 
Indexes:
    "replica_distribution_pkey" PRIMARY KEY, btree (client, num_of_replicas)

postgres=# select * from replica_distribution limit 2;
  client   | num_of_replicas | total_deal_size | unique_data_size 
-----------+-----------------+-----------------+------------------
 f01074655 |               1 |  31370441129984 |   29549374996480
 f01096236 |               1 |   1099511627776 |    1099511627776
(2 rows)
```

## Cid Sharing table

Table with details of data shared between clients. Each row contains:
* client id
* other client id
* amount of unique CIDs that both of these clients used in deals
* sum of sizes of all deals these clients made with shared CIDs

```
postgres=# \d cid_sharing;
                 Table "public.cid_sharing"
      Column      |  Type   | Collation | Nullable | Default 
------------------+---------+-----------+----------+---------
 client           | text    |           | not null | 
 other_client     | text    |           | not null | 
 unique_cid_count | integer |           | not null | 
 total_deal_size  | bigint  |           | not null | 
Indexes:
    "cid_sharing_pkey" PRIMARY KEY, btree (client, other_client)

postgres=# select * from cid_sharing limit 2;
  client   | other_client | unique_cid_count | total_deal_size 
-----------+--------------+------------------+-----------------
 f01471028 | f01762699    |             2355 | 168706315386880
 f01471028 | f01943349    |             2359 | 185336428756992
(2 rows)
```

## Aggregated Client Deals table

Table with deal data aggregated to 1h windows. Each row contains:
* client id
* window start as filecoin epoch (inclusive)
* window end as filecoin epoch (inclusive)
* sum of sizes of all deals this client made in this window

```
postgres=# \d aggregated_client_deals;
           Table "public.aggregated_client_deals"
     Column      |  Type   | Collation | Nullable | Default 
-----------------+---------+-----------+----------+---------
 client          | text    |           | not null | 
 term_start_from | integer |           | not null | 
 term_start_to   | integer |           | not null | 
 total_deal_size | bigint  |           | not null | 
Indexes:
    "aggregated_client_deals_pkey" PRIMARY KEY, btree (client, term_start_from)

postgres=# select * from aggregated_client_deals limit 2;
  client   | term_start_from | term_start_to | total_deal_size 
-----------+-----------------+---------------+-----------------
 f01074655 |         2608800 |       2608919 |    721554505728
 f01074655 |         2608920 |       2609039 |    549755813888
(2 rows)
```

# Development

If you're changing queries during development, you need to get queries metadata from DB for type safety checks.

If changing queries in `source-db`, run this in `source-db` directory:
```
env DATABASE_URL=$SOURCE_DATABASE_URL cargo sqlx prepare
```

If changing queries in `dest-db`, run this in `dest-db` directory:
```
env DATABASE_URL=$TARGET_DATABASE_URL cargo sqlx prepare
```

Commit the new files in `.sqlx` directories.