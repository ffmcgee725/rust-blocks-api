# Rust Project: Block by Date

This project provides an API for querying blockchain data by date and block number. The API supports multiple blockchain networks and allows you to fetch the block data by providing a timestamp or get the date by providing a block number.

## Environment Variables

Set the following environment variable before running the project:
```sh
cp .env.sample .env
```

Edit the .env file to include your database connection string:

```makefile
DATABASE_URL=<Connection string to a database named `rust-block-by-date`>
```

## Running the Project Locally

To run the project locally, ensure you have Rust installed \
also have diesel cli, you can check [Diesel Docs](https://diesel.rs/guides/getting-started.html) to see how to do it \
use the following command:

```bash
diesel migration run # This will run the existing migrations onto your database
cargo run # Runs your project in development mode
```

## Available Endpoints

1. `/block_by_date`

### Retrieve block data for a given timestamp (in seconds)

URL: `/block_by_date` \
Method: `GET` \
Query Parameters: \
  network_id: Chain identifier number of the supported chains (see list below). \
  timestamp: Timestamp (in seconds) of the block you want (pass 0 if you want the current block).

Supported `network_id` values: \
42161 => ARBITRUM\
43114 => AVALANCHE\
1313161554 => AURORA\
8453 => BASE\
42220 => CELO\
1 => ETHEREUM\
250 => FANTOM\
127 => POLYGON\
10 => OPTIMISM\
2020 => RONIN\
100 => XDAI

Example Request:
```http
GET /block_by_date?network_id=1&timestamp=1625097600
```

2. `/date_by_block`

### Retrieve timestamp for a given block number

URL: `/date_by_block`\
Method: `GET`\
Query Parameters:\
  network_id: Chain identifier number of the supported chains (see list above).\
  block: Block number you want the timestamp for (pass 0 for the current timestamp in seconds).

Example Request:
```http
GET /date_by_block?network_id=1&block=12345678
```

## Building the Project for Production
To build the project for production, use the following command:

```bash
cargo build --release
```

This will create an optimized build of the project in the target/release directory.

## Contributing
Feel free to open issues or submit pull requests if you find bugs or have suggestions for improvements.

## License
This project is licensed under the MIT License.
