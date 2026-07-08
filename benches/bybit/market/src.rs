#[path = "../prelude.rs"]
mod prelude;

use bc_exch_api_funcs::bybit::market::src::*;
use prelude::*;

fn src_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("src_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.src_a("SUIUSDT", 10, 0, 0));
    });
}

fn src_a_lch_2(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("src_a_lch_2", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.src_a("SUIUSDT", 100000, 0, 0));
    });
}

fn src_series_symbols_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
    c.bench_function("src_symbols_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.src_series_symbols(symbols.as_slice()));
    });
}

fn src_series_symbols_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
    c.bench_function("src_symbols_a_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.src_series_symbols_a(symbols.as_slice()));
    });
}

fn src_series_symbols_ao_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
    c.bench_function("src_symbols_ao_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.src_series_symbols_ao(symbols.as_slice()));
    });
}

fn src_symbols_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
    c.bench_function("src_symbols_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.src_symbols(symbols.as_slice(), 10, 0, 0));
    });
}

fn src_symbols_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
    c.bench_function("src_symbols_a_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.src_symbols_a(symbols.as_slice(), 10, 0, 0));
    });
}

criterion_group!(
    benches,
    src_a_lch_1,
    src_a_lch_2,
    src_series_symbols_lch_1,
    src_series_symbols_a_lch_1,
    src_series_symbols_ao_lch_1,
    src_symbols_lch_1,
    src_symbols_a_lch_1,
);
criterion_main!(benches);
