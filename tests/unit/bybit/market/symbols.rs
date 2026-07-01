use std::sync::LazyLock;

use bc_utils_lg::settings::SETTINGS;
use bc_utils_lg::settings::settings_from_json;
use tokio;

use bc_exch_api_funcs::bybit::exch_struct::BYBIT;
use bc_exch_api_funcs::bybit::market::symbols::*;

static S: LazyLock<SETTINGS> =
    LazyLock::new(|| settings_from_json("settings.json".into()).unwrap());
static EXCH: LazyLock<BYBIT<'_>> = LazyLock::new(|| BYBIT::new(&*S));

#[tokio::test]
async fn symbols_req_lch_1() {
    EXCH.symbols_req("", "", "").await.unwrap();
}

#[tokio::test]
async fn symbols_a_lch_1() {
    EXCH.symbols_a("", "", "").await.unwrap();
}
