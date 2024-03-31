use serde::{Deserialize, Serialize};

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
pub struct TokenInfo {
    /// Optional authority used to mint new tokens. The mint authority may only be provided during
    /// mint creation. If no mint authority is present then the mint has a fixed supply and no
    /// further tokens may be minted.
    pub mint_authority: String,
    /// Total supply of tokens.
    pub supply: u64,
    /// Number of base 10 digits to the right of the decimal place.
    pub decimals: u8,
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,
    /// Optional authority to freeze token accounts.
    pub freeze_authority: String,
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