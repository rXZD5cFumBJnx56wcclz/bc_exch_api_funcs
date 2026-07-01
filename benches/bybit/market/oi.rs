#[path = "../prelude.rs"]
mod prelude;

use bc_exch_api_funcs::bybit::market::oi::*;
use prelude::*;

fn oi_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("oi_req_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.oi_req("SUIUSDT", "5min", 0, 0, 1, ""));
    });
}

fn oi_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("oi_a_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.oi_a("SUIUSDT", "5min", 0, 0, 1, ""));
    });
}

criterion_group!(benches, oi_req_lch_1, oi_a_lch_1,);
criterion_main!(benches);
