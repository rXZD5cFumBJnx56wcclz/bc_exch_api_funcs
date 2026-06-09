use criterion::{Criterion, criterion_group, criterion_main};
use std::time::Duration;
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::market::oi::*;

fn oi_req_lch_1(c: &mut Criterion) {
    let dur = Duration::from_secs(3);
    let rtm = Runtime::new().unwrap();
    c.bench_function("oi_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| {
            oi_req(
                "https://api.bybit.com",
                "linear",
                "SUIUSDT",
                "5min",
                0,
                0,
                1,
                "",
                &dur,
            )
        });
    });
}

fn oi_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("oi_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| {
            oi_a(
                "https://api.bybit.com",
                "linear",
                "SUIUSDT",
                "5min",
                0,
                0,
                1,
                "",
                3,
                3,
            )
        });
    });
}

criterion_group!(benches, oi_req_lch_1, oi_a_lch_1,);
criterion_main!(benches);
