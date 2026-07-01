use std::sync::LazyLock;

use bc_utils_lg::settings::SETTINGS;
use bc_utils_lg::settings::settings_from_json;
use tokio;

use bc_exch_api_funcs::bybit::exch_struct::BYBIT;
use bc_exch_api_funcs::bybit::market::orderbook::*;

static S: LazyLock<SETTINGS> =
    LazyLock::new(|| settings_from_json("settings.json".into()).unwrap());
static EXCH: LazyLock<BYBIT<'_>> = LazyLock::new(|| BYBIT::new(&*S));

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
