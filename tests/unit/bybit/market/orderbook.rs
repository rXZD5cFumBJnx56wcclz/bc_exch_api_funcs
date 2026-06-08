use std::time::Duration;

use tokio;

use bc_exch_api_funcs::bybit::market::orderbook::*;

#[tokio::test]
async fn orderbook_req_lch_1() {
    println!(
        "{:#?}",
        orderbook_req(
            "https://api.bybit.com",
            "linear",
            "SUIUSDT",
            &10,
            &Duration::from_millis(5000u64),
        )
        .await
        .unwrap()
    );
}

#[tokio::test]
async fn orderbook_a_lch_1() {
    orderbook_a("https://api.bybit.com", "linear", "SUIUSDT", &10, &0, &0)
        .await
        .unwrap();
}

#[tokio::test]
async fn orderbooks_lch_1() {
    let symbols = &[
        "SUIUSDT".to_string(),
        "WALRUSUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    let _ = orderbooks("https://api.bybit.com", "linear", symbols, &10, &0).await;
}

#[tokio::test]
async fn orderbooks_a_lch_1() {
    let symbols = &[
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    orderbooks_a("https://api.bybit.com", "linear", symbols, &10, &0, &0)
        .await
        .unwrap();
}
