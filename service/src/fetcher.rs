use std::error::Error;

use mpl_token_metadata::accounts::Metadata;
use reqwest::Client;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::{Account, GenericTokenAccount};
use spl_token_2022::extension::{BaseStateWithExtensions, StateWithExtensionsOwned};
use spl_token_2022::state::Mint;

use entity::token::{Holder, TokenInfo, TokenMetadata};
use utils::RpcUtils;

pub struct Fetcher;

static PRICE_FETCH_URL: &str = "https://price.jup.ag/v4/price";

impl Fetcher {
    pub async fn fetch_price(address: &str) -> Result<Option<f64>, Box<dyn Error>> {
        let response_text = Self::send_price_request(address).await?;
        Self::parse_price_response(address, &response_text)
    }

    pub async fn fetch_token_metadata(mint_address: &str) -> TokenMetadata {
        let client = RpcUtils::get_rpc_client("MAINNET_PUBLIC_URL");
        let mint_pubkey = mint_address.parse::<Pubkey>().expect("Invalid mint address");
        let metadata_pda = Metadata::find_pda(&mint_pubkey).0;
        let account_data = client.get_account_data(&metadata_pda).expect("Failed to get account data");
        let metadata = Metadata::safe_deserialize(&mut account_data.as_slice()).expect("Failed to deserialize metadata");

        TokenMetadata {
            update_authority: metadata.update_authority.to_string(),
            mint: metadata.mint.to_string(),
            name: metadata.name.trim_matches('\u{0000}').parse().unwrap(),
            symbol: metadata.symbol.trim_matches('\u{0000}').parse().unwrap(),
            uri: metadata.uri.trim_matches('\u{0000}').parse().unwrap(),
            seller_fee_basis_points: metadata.seller_fee_basis_points,
            is_mutable: metadata.is_mutable,
        }
    }

    pub async fn fetch_top_holders(mint_address: &str, paging_num: usize) -> Vec<Holder> {
        let client = RpcUtils::get_rpc_client("MAINNET_PRIVATE_URL");
        let mint_pubkey = mint_address.parse::<Pubkey>().expect("Invalid mint address");
        let total_supply = client.get_token_supply(&mint_pubkey).unwrap().ui_amount.unwrap();
        let accounts = client.get_token_largest_accounts(&mint_pubkey).unwrap();

        let mut holders = Vec::<Holder>::new();

        for address in accounts.iter().take(paging_num) {
            let token_account_pubkey = address.address.parse::<Pubkey>().expect("Invalid mint address");
            let account_data = client.get_account_data(&token_account_pubkey).unwrap();
            let owner_address = Account::unpack_account_owner(&account_data).unwrap();
            let ratio = address.amount.ui_amount.unwrap() / total_supply * 100.0;
            let _address = address.clone();

            let holder = Holder {
                address: _address.address,
                owner_address: owner_address.to_string(),
                amount: _address.amount.amount,
                ui_amount: _address.amount.ui_amount.unwrap(),
                ui_amount_string: _address.amount.ui_amount_string,
                percentage: ratio,
            };

            holders.push(holder);
        }

        holders
    }

    pub async fn fetch_token_extension_info(mint_address: &str) -> Vec<String> {
        let client = RpcUtils::get_rpc_client("MAINNET_PRIVATE_URL");
        let mint_pubkey = mint_address.parse::<Pubkey>().expect("Invalid mint address");
        let account_data = client.get_account_data(&mint_pubkey).unwrap();
        let result = StateWithExtensionsOwned::<Mint>::unpack(account_data).unwrap().get_extension_types().unwrap();

        let mut extensions = Vec::<String>::new();

        for extension in result {
            extensions.push(format!("{:?}", extension));
        }

        extensions
    }

    pub async fn fetch_token_info(mint_address: &str) -> TokenInfo {
        let client = RpcUtils::get_rpc_client("MAINNET_PRIVATE_URL");
        let mint_pubkey = mint_address.parse::<Pubkey>().expect("Invalid mint address");
        let account_data = client.get_account_data(&mint_pubkey).unwrap();
        let mint = Mint::unpack(&account_data).unwrap();

        TokenInfo {
            mint_authority: mint.mint_authority.map_or("null".to_string(), |s| s.to_string()),
            supply: mint.supply,
            decimals: mint.decimals,
            is_initialized: mint.is_initialized,
            freeze_authority: mint.freeze_authority.map_or("null".to_string(), |s| s.to_string()),
        }
    }

    async fn send_price_request(address: &str) -> Result<String, Box<dyn Error>> {
        let client = Client::new();
        let response_text = client.get(PRICE_FETCH_URL)
            .query(&[("ids", address)])
            .send()
            .await?
            .text()
            .await?;
        Ok(response_text)
    }

    fn parse_price_response(address: &str, response_text: &str) -> Result<Option<f64>, Box<dyn Error>> {
        let res: Value = serde_json::from_str(&response_text)?;
        Ok(res["data"][address]["price"].as_f64())
    }
}