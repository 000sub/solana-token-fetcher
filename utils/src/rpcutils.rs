use std::env;

use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;

pub struct RpcUtils;

impl RpcUtils {
    pub fn get_rpc_client(url: &str) -> RpcClient {
        dotenv().ok();
        let rpc_url = env::var(url).ok().unwrap();
        RpcClient::new(rpc_url)
    }
}