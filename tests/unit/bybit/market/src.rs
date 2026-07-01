#![warn(unused_must_use)]

use bc_exch_api_funcs::bybit::market::src::*;

use crate::unit::bybit::prelude::*;

#[tokio::test]
async fn src_lch_1() {
    EXCH.src("SUIUSDT", 50_000, 0, 0).await.unwrap();
}

#[tokio::test]
async fn src_a_lch_1() {
    EXCH.src_a("SUIUSDT", 50_000, 0, 0).await.unwrap();
}

#[tokio::test]
async fn src_series_symbols_lch_1() {
    EXCH.src_series_symbols(&["SUIUSDT".to_string(), "ETHUSDT".to_string()])
        .await;
}

#[tokio::test]
async fn src_series_symbols_a_lch_1() {
    EXCH.src_series_symbols_a(&["SUIUSDT".to_string(), "ETHUSDT".to_string()])
        .await
        .unwrap();
}

#[tokio::test]
async fn src_series_symbols_ao_lch_1() {
    EXCH.src_series_symbols_ao(&["SUIUSDT".to_string(), "ETHUSDT".to_string()])
        .await
        .unwrap();
}

#[tokio::test]
async fn src_symbols_lch_1() {
    EXCH.src_symbols(
        &["SUIUSDT".to_string(), "ETHUSDT".to_string()],
        50_000,
        0,
        0,
    )
    .await;
}

#[tokio::test]
async fn src_symbols_a_lch_1() {
    EXCH.src_symbols_a(
        &["SUIUSDT".to_string(), "ETHUSDT".to_string()],
        50_000,
        0,
        0,
    )
    .await
    .unwrap();
}
