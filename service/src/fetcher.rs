use std::error::Error;

use reqwest::Client;
use serde_json::Value;

pub struct Fetcher;

static PRICE_FETCH_URL: &str = "https://price.jup.ag/v4/price";

impl Fetcher {
    pub async fn fetch_price(address: &str) -> Result<Option<f64>, Box<dyn Error>> {
        let response_text = Self::send_price_request(address).await?;
        Self::parse_price_response(address, &response_text)
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