#[path = "../prelude.rs"]
mod prelude;

use bc_exch_api_funcs::bybit::market::orderbook::*;
use prelude::*;

fn orderbook_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("orderbook_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.orderbook_req("SUIUSDT", 10));
    });
}

fn orderbook_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("orderbook_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.orderbook_a("SUIUSDT", 10));
    });
}

fn orderbooks_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = &["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
    c.bench_function("orderbooks_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.orderbooks(symbols, 10));
    });
}

fn orderbooks_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = &["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
    c.bench_function("orderbooks_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.orderbooks_a(symbols, 10));
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
