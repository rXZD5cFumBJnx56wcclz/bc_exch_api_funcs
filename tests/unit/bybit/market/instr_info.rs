use bc_exch_api_funcs::bybit::market::instr_info::*;

use crate::unit::bybit::prelude::*;

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
