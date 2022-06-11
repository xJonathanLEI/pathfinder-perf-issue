use std::time::Instant;

use starknet::providers::jsonrpc::{
    models::{BlockHashOrTag, BlockTag},
    HttpTransport, JsonRpcClient,
};
use url::Url;

#[tokio::main]
async fn main() {
    let args = std::env::args().into_iter().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        panic!("RPC URL not found");
    }
    dbg!(&args);
    let rpc_url = Url::parse(&args[0]).unwrap();

    let client = JsonRpcClient::new(HttpTransport::new(rpc_url));

    let start_block = client
        .get_block_by_hash(&BlockHashOrTag::Tag(BlockTag::Latest))
        .await
        .unwrap();

    println!("Start block number: {}", start_block.metadata.block_number);

    loop {
        let start_time = Instant::now();
        let current_block = client
            .get_block_by_hash(&BlockHashOrTag::Tag(BlockTag::Latest))
            .await
            .unwrap();
        let time_elapsed = Instant::now() - start_time;

        println!(
            "Block: #{}. Time: {}ms",
            current_block.metadata.block_number,
            time_elapsed.as_millis()
        );

        if current_block.metadata.block_number != start_block.metadata.block_number {
            break;
        }
    }
}
