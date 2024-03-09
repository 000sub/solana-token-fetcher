use service::Fetcher;

static USDC_ADDRESS: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

#[tokio::test]
pub async fn fetch_price_test() {
    let price = Fetcher::fetch_price(USDC_ADDRESS)
        .await
        .ok()
        .unwrap()
        .unwrap();

    assert_eq!(price, 1.0);
}