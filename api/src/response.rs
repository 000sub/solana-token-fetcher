use serde::{Deserialize, Serialize};

use entity::token::{Holder, TokenInfo, TokenMetadata};

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub mint: String,
    pub metadata: TokenMetadata,
    pub extensions: Vec<String>,
    pub info: TokenInfo,
    pub top_holders: Vec<Holder>,
}

