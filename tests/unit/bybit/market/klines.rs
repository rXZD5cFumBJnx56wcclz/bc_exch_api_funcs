#![warn(unused_must_use)]

use bc_exch_api_funcs::bybit::market::klines::*;

use crate::unit::bybit::prelude::*;

#[tokio::test]
async fn klines_req_lch_1() {
    EXCH.klines_req("SUIUSDT", 100_000, 0, 0).await.unwrap();
}

#[tokio::test]
async fn klines_a_lch_1() {
    EXCH.klines_a("SUIUSDT", 10, 0, 0).await.unwrap();
}

#[tokio::test]
async fn klines_a_res_1() {
    let res = EXCH.klines_a("SUIUSDT", 99_999, 0, 0).await.unwrap();
    if res[999][0] > res[1000][0] || res.len() != 99_999 {
        dbg!(res.len(), res[999][0], res[1000][0]);
        panic!();
    }
}

#[tokio::test]
async fn klines_a_res_2() {
    let res = EXCH
        .klines_a("BTCUSDT", 1100, 1669852800000, 1671062400000)
        .await
        .unwrap();
    if res[999][0] > res[1000][0] || res.len() != 1100 {
        dbg!(res.len(), res[999][0], res[1000][0]);
        panic!();
    }
}

#[tokio::test]
async fn klines_a_res_3() {
    let res = EXCH
        .klines_a("BTCUSDT", 1100, 1669852800000, 1671062400000)
        .await
        .unwrap();
    if res[999][0] > res[1000][0] || res.len() != 1100 {
        dbg!(res.len(), res[999][0], res[1000][0]);
        panic!();
    }
}

#[tokio::test]
async fn kline_symbols_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    let _ = EXCH.kline_symbols(symbols.as_slice()).await;
}

#[tokio::test]
async fn kline_symbols_a_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    EXCH.kline_symbols_a(symbols.as_slice()).await.unwrap();
}

#[tokio::test]
async fn kline_symbols_ao_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    EXCH.kline_symbols_ao(symbols.as_slice()).await.unwrap();
}

#[tokio::test]
async fn klines_symbols_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    let _ = EXCH.klines_symbols(symbols.as_slice(), 10, 0, 0).await;
}

#[tokio::test]
async fn klines_symbols_a_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    EXCH.klines_symbols_a(symbols.as_slice(), 10, 0, 0)
        .await
        .unwrap();
}
