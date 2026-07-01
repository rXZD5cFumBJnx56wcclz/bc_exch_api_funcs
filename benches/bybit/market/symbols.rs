#[path = "../prelude.rs"]
mod prelude;

use bc_exch_api_funcs::bybit::market::symbols::*;
use prelude::*;

fn symbols_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("symbols_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.symbols_req("", "", ""));
    });
}

fn symbols_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("symbols_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.symbols_a("", "", ""));
    });
}

criterion_group!(benches, symbols_req_lch_1, symbols_a_lch_1,);
criterion_main!(benches);
