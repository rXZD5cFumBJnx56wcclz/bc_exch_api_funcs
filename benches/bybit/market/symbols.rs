use std::time::Duration;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::market::symbols::*;

fn symbols_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let dur = Duration::from_secs(3);
    c.bench_function("symbols_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| symbols_req(
            "https://api.bybit.com", 
            "linear",
            "",
            "",
            "",
            &dur,
        ));
    });
}

fn symbols_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("symbols_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| symbols_a(
            "https://api.bybit.com", 
            "linear",
            "",
            "",
            "",
            &3,
            &3,
        ));
    });
}

criterion_group!(
    benches, 
    symbols_req_lch_1,
    symbols_a_lch_1,
);
criterion_main!(benches);