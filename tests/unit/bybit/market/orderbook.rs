use bc_exch_api_funcs::bybit::market::orderbook::*;

use crate::unit::bybit::prelude::*;

#[tokio::test]
async fn orderbook_req_lch_1() {
    println!("{:#?}", EXCH.orderbook_req("SUIUSDT", 10,).await.unwrap());
}

#[tokio::test]
async fn orderbook_a_lch_1() {
    EXCH.orderbook_a("SUIUSDT", 10).await.unwrap();
}

#[tokio::test]
async fn orderbooks_lch_1() {
    let symbols = &[
        "SUIUSDT".to_string(),
        "WALRUSUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    let _ = EXCH.orderbooks(symbols, 10).await;
}

#[tokio::test]
async fn orderbooks_a_lch_1() {
    let symbols = &[
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    EXCH.orderbooks_a(symbols, 10).await.unwrap();
}
