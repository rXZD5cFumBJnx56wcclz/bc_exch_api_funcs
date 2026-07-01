use std::sync::LazyLock;

use bc_utils_lg::settings::SETTINGS;
use bc_utils_lg::settings::settings_from_json;
use tokio;

use bc_exch_api_funcs::bybit::exch_struct::BYBIT;
use bc_exch_api_funcs::bybit::market::instr_info::*;

static S: LazyLock<SETTINGS> =
    LazyLock::new(|| settings_from_json("settings.json".into()).unwrap());
static EXCH: LazyLock<BYBIT<'_>> = LazyLock::new(|| BYBIT::new(&*S));

#[tokio::test]
async fn instr_info_req_lch_1() {
    EXCH.instr_info_req("BTCUSDT", "", "", 1, "").await.unwrap();
}

#[tokio::test]
async fn instr_info_lch_1() {
    EXCH.instr_info("SUIUSDT", "", "").await.unwrap();
}

#[tokio::test]
async fn instrs_info_lch_1() {
    EXCH.instrs_info(
        &[
            "SUIUSDT".to_string(),
            "UNIUSDT".to_string(),
            "ETHUSDT".to_string(),
        ],
        "",
        "",
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn instrs_info_a_lch_1() {
    EXCH.instrs_info_a(
        &[
            "SUIUSDT".to_string(),
            "UNIUSDT".to_string(),
            "ETHUSDT".to_string(),
        ],
        "",
        "",
    )
    .await
    .unwrap();
}
