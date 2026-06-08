use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::market::klines::*;

fn klines_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let dur = Duration::from_secs(3);
    c.bench_function("klines_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| {
            klines_req(
                "https://api.bybit.com",
                "linear",
                "SUIUSDT",
                "1",
                &10,
                &0,
                &0,
                &dur,
            )
        });
    });
}

fn klines_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("klines_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| {
            klines_a(
                "https://api.bybit.com",
                "linear",
                "SUIUSDT",
                "1",
                &10,
                &0,
                &0,
                &3,
                &3,
            )
        });
    });
}

fn klines_a_lch_2(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("klines_a_lch_2", |b| {
        b.to_async(&rtm).iter(|| {
            klines_a(
                "https://api.bybit.com",
                "linear",
                "SUIUSDT",
                "1",
                &100000,
                &0,
                &0,
                &3,
                &3,
            )
        });
    });
}

fn kline_symbols_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    c.bench_function("kline_symbols_lch_1", |b| {
        b.to_async(&rtm).iter(|| {
            kline_symbols(
                "https://api.bybit.com",
                "linear",
                symbols.as_slice(),
                "1",
                &3,
            )
        });
    });
}

fn kline_symbols_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    c.bench_function("kline_symbols_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| {
            kline_symbols_a(
                "https://api.bybit.com",
                "linear",
                symbols.as_slice(),
                "1",
                &3,
                &3,
            )
        });
    });
}

fn kline_symbols_ao_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    c.bench_function("kline_symbols_ao_lch_1", |b| {
        b.to_async(&rtm).iter(|| {
            kline_symbols_ao(
                "https://api.bybit.com",
                "linear",
                symbols.as_slice(),
                "1",
                &3,
                &3,
            )
        });
    });
}

fn klines_symbols_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    c.bench_function("klines_symbols_lch_1", |b| {
        b.to_async(&rtm).iter(|| {
            klines_symbols(
                "https://api.bybit.com",
                "linear",
                symbols.as_slice(),
                "1",
                &10,
                &0,
                &0,
                &3,
            )
        });
    });
}

fn klines_symbols_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec![
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    c.bench_function("klines_symbols_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| {
            klines_symbols_a(
                "https://api.bybit.com",
                "linear",
                symbols.as_slice(),
                "1",
                &10,
                &0,
                &0,
                &3,
                &3,
            )
        });
    });
}

criterion_group!(
    benches,
    klines_req_lch_1,
    klines_a_lch_1,
    klines_a_lch_2,
    kline_symbols_lch_1,
    kline_symbols_a_lch_1,
    kline_symbols_ao_lch_1,
    klines_symbols_lch_1,
    klines_symbols_a_lch_1,
);
criterion_main!(benches);
