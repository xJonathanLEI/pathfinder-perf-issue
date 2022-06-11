use std::time::Instant;

use rand::{thread_rng, Rng};
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

    let latest_block = client
        .get_block_by_number(&BlockNumOrTag::Tag(BlockTag::Latest))
        .await
        .unwrap();

    let mut rng = thread_rng();

    for _ in 0..10 {
        let random_block_number = rng.gen_range(0..latest_block.metadata.block_number);

        let start_time = Instant::now();
        let current_block = client
            .get_block_by_number(&BlockNumOrTag::Number(random_block_number))
            .await
            .unwrap();
        let time_elapsed = Instant::now() - start_time;

        println!(
            "Current block: #{}. Time: {}ms",
            current_block.metadata.block_number,
            time_elapsed.as_millis()
        );
    }
}
