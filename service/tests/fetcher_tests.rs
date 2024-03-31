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

#[tokio::test(flavor = "multi_thread")]
pub async fn fetch_metadata_test() {
    let result = Fetcher::fetch_token_metadata(USDC_ADDRESS)
        .await.ok().unwrap();

    assert_eq!(result.name, "USD Coin");
    assert_eq!(result.symbol, "USDC");
}

#[tokio::test(flavor = "multi_thread")]
pub async fn fetch_top_holders_test() {
    let result = Fetcher::fetch_top_holders("EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm", 5)
        .await.ok().unwrap();

    assert_eq!(result.len(), 5);
    assert_eq!(result.get(0).unwrap().owner_address, "5tzFkiKscXHK5ZXCGbXZxdw7gTjjD1mBwuoFbhUvuAi9");
}

#[tokio::test(flavor = "multi_thread")]
pub async fn fetch_top_holders_test_fail() {
    match Fetcher::fetch_top_holders("invalid_address", 5).await {
        Ok(result) => {
            panic!("test failed");
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    };
}

#[tokio::test(flavor = "multi_thread")]
pub async fn fetch_extension_test() {
    let result = Fetcher::fetch_token_extension_info("ADTv1dri1ymmQjndQPMsmWUwcWbfvPS7siyj7V7EFuP5")
        .await.ok().unwrap();

    assert_eq!(result.len(), 6);
    assert_eq!(result.get(0).unwrap(), "TransferFeeConfig");
}

#[tokio::test(flavor = "multi_thread")]
pub async fn fetch_token_info_test() {
    let result = Fetcher::fetch_token_info(USDC_ADDRESS)
        .await.ok().unwrap();

    assert_eq!(result.mint_authority, "2wmVCSfPxGPjrnMMn7rchp4uaeoTqN39mXFC2zhPdri9");
}