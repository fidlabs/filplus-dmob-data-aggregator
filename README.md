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