#![warn(unused_must_use)]

use std::time::Duration;

use bc_exch_api_funcs::bybit::market::klines::*;

#[tokio::test]
async fn klines_req_lch_1() {
    klines_req(
        "https://api.bybit.com",
        "linear",
        "SUIUSDT",
        "1",
        100_000,
        0,
        0,
        &Duration::from_millis(5000u64),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn klines_a_lch_1() {
    klines_a(
        "https://api.bybit.com",
        "linear",
        "SUIUSDT",
        "1",
        10,
        0,
        0,
        0,
        0,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn klines_a_res_1() {
    let res = klines_a(
        "https://api.bybit.com",
        "linear",
        "SUIUSDT",
        "1",
        99_999,
        0,
        0,
        0,
        0,
    )
    .await
    .unwrap();
    if res[999][0] > res[1000][0] || res.len() != 99_999 {
        dbg!(res.len(), res[999][0], res[1000][0]);
        panic!();
    }
}

#[tokio::test]
async fn klines_a_res_2() {
    let res = klines_a(
        "https://api.bybit.com",
        "linear",
        "BTCUSDT",
        "1",
        1100,
        1669852800000,
        1671062400000,
        0,
        0,
    )
    .await
    .unwrap();
    if res[999][0] > res[1000][0] || res.len() != 1100 {
        dbg!(res.len(), res[999][0], res[1000][0]);
        panic!();
    }
}

#[tokio::test]
async fn klines_a_res_3() {
    let res = klines_a(
        "https://api.bybit.com",
        "linear",
        "BTCUSDT",
        "1",
        1100,
        1669852800000,
        1671062400000,
        0,
        0,
    )
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
    let _ = kline_symbols(
        "https://api.bybit.com",
        "linear",
        symbols.as_slice(),
        "1",
        0,
    )
    .await;
}

#[tokio::test]
async fn kline_symbols_a_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    kline_symbols_a(
        "https://api.bybit.com",
        "linear",
        symbols.as_slice(),
        "1",
        0,
        0,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn kline_symbols_ao_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    kline_symbols_ao(
        "https://api.bybit.com",
        "linear",
        symbols.as_slice(),
        "1",
        0,
        0,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn klines_symbols_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    let _ = klines_symbols(
        "https://api.bybit.com",
        "linear",
        symbols.as_slice(),
        "1",
        10,
        0,
        0,
        0,
    )
    .await;
}

#[tokio::test]
async fn klines_symbols_a_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    klines_symbols_a(
        "https://api.bybit.com",
        "linear",
        symbols.as_slice(),
        "1",
        10,
        0,
        0,
        0,
        0,
    )
    .await
    .unwrap();
}
