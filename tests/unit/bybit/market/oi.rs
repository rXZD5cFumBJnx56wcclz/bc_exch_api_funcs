use std::sync::LazyLock;

use bc_utils_lg::settings::SETTINGS;
use bc_utils_lg::settings::settings_from_json;
use tokio;

use bc_exch_api_funcs::bybit::exch_struct::BYBIT;
use bc_exch_api_funcs::bybit::market::oi::*;

static S: LazyLock<SETTINGS> =
    LazyLock::new(|| settings_from_json("settings.json".into()).unwrap());
static EXCH: LazyLock<BYBIT<'_>> = LazyLock::new(|| BYBIT::new(&*S));

#[tokio::test]
async fn oi_req_lch_1() {
    println!("{:#?}", EXCH.oi_req("SUIUSDT", "5min", 0, 0, 1, "",).await);
}

#[tokio::test]
async fn oi_a_lch_1() {
    println!(
        "{:#?}",
        EXCH.oi_a("SUIUSDT", "5min", 0, 0, 1, "",).await.unwrap()
    );
}
