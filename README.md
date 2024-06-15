[Diesel Docs](https://diesel.rs/guides/getting-started.html)

## TODO
Simple flow
  -> Receive request (networkId, timestamp) [] // web::Query<Option<String>> para query param ? 
  -> Fetch networkConfigs (Trait for each existing network, defines the interface of available methods) []
    -> Throw if non existing network
  -> If block is already stored in database, return []
  -> From networkConfig, retrieve subgraphUrl and query subgraph on block for timestamp (or latest if provided timestamp = 0) []


# ETH CONFIG
subgraphURL => https://thegraph.com/hosted-service/subgraph/snowfork/ethereum-blocks
block by date query =>
`{blocks(first: 1, orderBy: number, orderDirection: asc, where: { timestamp_gte: ${timestamp}, timestamp_lt: ${
    timestamp + 60
  } }) { id number timestamp }}`;