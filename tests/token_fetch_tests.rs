use std::env;

use dotenv::dotenv;
use mpl_token_metadata::accounts::Metadata;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use spl_token::state::{Account, GenericTokenAccount};

fn get_rpc_client(url: &str) -> RpcClient {
    dotenv().ok();
    let rpc_url = env::var(url).ok().unwrap();
    RpcClient::new(rpc_url)
}

#[derive(Serialize, Deserialize)]
pub struct TokenMetadata {
    pub update_authority: String,
    pub mint: String,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub is_mutable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Holder {
    pub address: String,
    pub owner_address: String,
    pub amount: String,
    pub ui_amount: f64,
    pub ui_amount_string: String,
    pub percentage: f64,
}

#[test]
pub fn get_metadata() {
    let client = get_rpc_client("MAINNET_PUBLIC_URL");
    let mint_address = "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm";
    let mint_pubkey = mint_address.parse::<Pubkey>().expect("Invalid mint address");
    let metadata_pda = Metadata::find_pda(&mint_pubkey).0;
    let account_data = client.get_account_data(&metadata_pda).expect("Failed to get account data");
    let metadata = Metadata::safe_deserialize(&mut account_data.as_slice()).expect("Failed to deserialize metadata");

    let metadata = TokenMetadata {
        update_authority: metadata.update_authority.to_string(),
        mint: metadata.mint.to_string(),
        name: metadata.name.trim_matches('\u{0000}').parse().unwrap(),
        symbol: metadata.symbol.trim_matches('\u{0000}').parse().unwrap(),
        uri: metadata.uri.trim_matches('\u{0000}').parse().unwrap(),
        seller_fee_basis_points: metadata.seller_fee_basis_points,
        is_mutable: metadata.is_mutable,
    };

    let result = serde_json::to_string(&metadata).unwrap();
    println!("{}", result);
}

#[test]
pub fn print_top_5_wif_holders() {
    let client = get_rpc_client("MAINNET_PRIVATE_URL");
    let mint_address = "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm";
    let mint_pubkey = mint_address.parse::<Pubkey>().expect("Invalid mint address");
    let total_supply = client.get_token_supply(&mint_pubkey).unwrap().ui_amount.unwrap();
    let accounts = client.get_token_largest_accounts(&mint_pubkey).unwrap();


    for (index, address) in accounts.iter().take(5).enumerate() {
        let token_account_pubkey = address.address.parse::<Pubkey>().expect("Invalid mint address");
        let account_data = client.get_account_data(&token_account_pubkey).unwrap();
        let owner_address = Account::unpack_account_owner(&account_data).unwrap();
        let ratio = address.amount.ui_amount.unwrap() / total_supply as f64 * 100.0;
        // println!("{}: Address = {}, Balance = {}, Ratio = {:.2}%", index + 1, owner_address, address.amount.ui_amount_string, ratio);
        let _address = address.clone();

        let holder = Holder {
            address: _address.address,
            owner_address: owner_address.to_string(),
            amount: _address.amount.amount,
            ui_amount: _address.amount.ui_amount.unwrap(),
            ui_amount_string: _address.amount.ui_amount_string,
            percentage: ratio,
        };

        let result = serde_json::to_string(&holder).unwrap();
        println!("{}", result);
    }
}