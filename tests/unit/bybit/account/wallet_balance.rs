use std::sync::LazyLock;

use bc_utils_lg::settings::SETTINGS;
use bc_utils_lg::settings::settings_from_json;
use reqwest::Client;
use tokio;

use bc_exch_api_funcs::bybit::account::wallet_balance::*;
use bc_exch_api_funcs::bybit::exch_struct::BYBIT;

static S: LazyLock<SETTINGS> =
    LazyLock::new(|| settings_from_json("settings.json".into()).unwrap());
static EXCH: LazyLock<BYBIT<'_>> = LazyLock::new(|| BYBIT::new(&*S));

#[tokio::test]
async fn wallet_balance_req_lch_1() {
    println!(
        "{:#?}",
        EXCH.wallet_balance_req(&Client::new(), "UNIFIED", "USDT",)
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn wallet_balance_a_lch_1() {
    println!(
        "{:#?}",
        EXCH.wallet_balance_a(&Client::new(), "UNIFIED", "USDT", 0,)
            .await
            .unwrap()
    );
}
