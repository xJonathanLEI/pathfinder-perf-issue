use std::time::Instant;

use starknet::providers::jsonrpc::{
    models::{BlockNumOrTag, BlockTag},
    HttpTransport, JsonRpcClient,
};
use url::Url;

#[tokio::main]
async fn main() {
    let args = std::env::args().into_iter().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        panic!("RPC URL not found");
    }
    let rpc_url = Url::parse(&args[0]).unwrap();

    let client = JsonRpcClient::new(HttpTransport::new(rpc_url));

    loop {
        let start_time = Instant::now();
        let start_block = client
            .get_block_by_number(&BlockNumOrTag::Tag(BlockTag::Latest))
            .await
            .unwrap();
        let time_elapsed = Instant::now() - start_time;

        println!(
            "Current block number: #{}, Time: {}ms",
            start_block.metadata.block_number,
            time_elapsed.as_millis()
        );
        println!(
            "Querying for the next block #{} directly",
            start_block.metadata.block_number + 1
        );

        loop {
            let start_time = Instant::now();
            let next_block = match client
                .get_block_by_number(&BlockNumOrTag::Number(
                    start_block.metadata.block_number + 1,
                ))
                .await
            {
                Ok(block) => block,
                Err(_) => continue, // The block likely isn't available yet
            };
            let time_elapsed = Instant::now() - start_time;

            println!(
                "Block: #{}. Time: {}ms",
                next_block.metadata.block_number,
                time_elapsed.as_millis()
            );

            if next_block.metadata.block_number != start_block.metadata.block_number {
                break;
            }
        }
    }
}
