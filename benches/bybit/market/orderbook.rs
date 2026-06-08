use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::market::orderbook::*;

fn orderbook_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let dur = Duration::from_secs(3);
    c.bench_function("orderbook_req_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| orderbook_req("https://api.bybit.com", "linear", "SUIUSDT", &10, &dur));
    });
}

fn orderbook_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("orderbook_a_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| orderbook_a("https://api.bybit.com", "linear", "SUIUSDT", &10, &3, &3));
    });
}

fn orderbooks_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = &[
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    c.bench_function("orderbooks_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| orderbooks("https://api.bybit.com", "linear", symbols, &10, &3));
    });
}

fn orderbooks_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = &[
        "SUIUSDT".to_string(),
        "ETHUSDT".to_string(),
        "ATOMUSDT".to_string(),
    ];
    c.bench_function("orderbooks_a_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| orderbooks_a("https://api.bybit.com", "linear", symbols, &10, &3, &3));
    });
}

criterion_group!(
    benches,
    orderbook_req_lch_1,
    orderbook_a_lch_1,
    orderbooks_lch_1,
    orderbooks_a_lch_1,
);
criterion_main!(benches);
