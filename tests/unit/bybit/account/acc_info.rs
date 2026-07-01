use std::sync::LazyLock;

use bc_utils_lg::settings::SETTINGS;
use bc_utils_lg::settings::settings_from_json;
use reqwest::Client;
use tokio;

use bc_exch_api_funcs::bybit::account::acc_info::*;
use bc_exch_api_funcs::bybit::exch_struct::BYBIT;

static S: LazyLock<SETTINGS> =
    LazyLock::new(|| settings_from_json("settings.json".into()).unwrap());
static EXCH: LazyLock<BYBIT<'_>> = LazyLock::new(|| BYBIT::new(&*S));

#[tokio::test]
async fn acc_info_req_lch_1() {
    println!("{:#?}", EXCH.acc_info_req(&Client::new(),).await.unwrap());
}

#[tokio::test]
async fn acc_info_a_lch_1() {
    println!("{:#?}", EXCH.acc_info_a(&Client::new(),).await.unwrap());
}
