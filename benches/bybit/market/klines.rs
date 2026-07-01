#[path = "../prelude.rs"]
mod prelude;

use bc_exch_api_funcs::bybit::market::klines::*;
use prelude::*;

fn klines_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("klines_req_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.klines_req("SUIUSDT", 10, 0, 0));
    });
}

fn klines_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("klines_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.klines_a("SUIUSDT", 10, 0, 0));
    });
}

fn klines_a_lch_2(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("klines_a_lch_2", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.klines_a("SUIUSDT", 100000, 0, 0));
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
        b.to_async(&rtm)
            .iter(|| EXCH.kline_symbols(symbols.as_slice()));
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
        b.to_async(&rtm)
            .iter(|| EXCH.kline_symbols_a(symbols.as_slice()));
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
        b.to_async(&rtm)
            .iter(|| EXCH.kline_symbols_ao(symbols.as_slice()));
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
        b.to_async(&rtm)
            .iter(|| EXCH.klines_symbols(symbols.as_slice(), 10, 0, 0));
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
        b.to_async(&rtm)
            .iter(|| EXCH.klines_symbols_a(symbols.as_slice(), 10, 0, 0));
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
