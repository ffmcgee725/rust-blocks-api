[Diesel Docs](https://diesel.rs/guides/getting-started.html)

## TODO

Simple flow
-> Receive request (networkId, timestamp) [X]
-> Fetch networkConfigs (Trait for each existing network, defines the interface of available methods) [X]
-> Throw if non existing network [X]
-> If block is already stored in database, return []
-> From networkConfig, retrieve subgraphUrl and query subgraph on block for timestamp (or latest if provided timestamp = 0) [X]
