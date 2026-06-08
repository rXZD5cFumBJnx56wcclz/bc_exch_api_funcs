use std::time::Duration;

use tokio;

use bc_exch_api_funcs::bybit::market::instr_info::*;

#[tokio::test]
async fn instr_info_req_lch_1() {
    instr_info_req(
        "https://api.bybit.com",
        "linear",
        "BTCUSDT",
        "",
        "",
        &1,
        "",
        &Duration::from_millis(5000u64),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn instr_info_lch_1() {
    instr_info("https://api.bybit.com", "linear", "SUIUSDT", "", "", &5000)
        .await
        .unwrap();
}

#[tokio::test]
async fn instrs_info_lch_1() {
    instrs_info(
        "https://api.bybit.com",
        "linear",
        &[
            "SUIUSDT".to_string(),
            "UNIUSDT".to_string(),
            "ETHUSDT".to_string(),
        ],
        "",
        "",
        &5000,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn instrs_info_a_lch_1() {
    instrs_info_a(
        "https://api.bybit.com",
        "linear",
        &[
            "SUIUSDT".to_string(),
            "UNIUSDT".to_string(),
            "ETHUSDT".to_string(),
        ],
        "",
        "",
        &5000,
        &6000,
    )
    .await
    .unwrap();
}
