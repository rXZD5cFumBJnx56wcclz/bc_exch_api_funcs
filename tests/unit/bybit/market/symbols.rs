use std::time::Duration;

use bc_exch_api_funcs::bybit::market::symbols::*;

#[tokio::test]
async fn symbols_req_lch_1() {
    symbols_req(
        "https://api.bybit.com", 
        "linear",
        "",
        "",
        "",
        &Duration::from_secs(5),
    )
        .await
        .unwrap();
}

#[tokio::test]
async fn symbols_a_lch_1() {
    symbols_a(
        "https://api.bybit.com", 
        "linear",
        "",
        "",
        "",
        &0,
        &0,
    )
        .await.unwrap();
}
