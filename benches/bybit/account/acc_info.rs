#[path = "../prelude.rs"]
mod prelude;

use bc_exch_api_funcs::bybit::account::acc_info::*;
use prelude::*;

fn acc_info_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();

    c.bench_function("acc_info_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.acc_info_req());
    });
}

fn acc_info_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();

    c.bench_function("acc_info_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.acc_info_a());
    });
}

criterion_group!(benches, acc_info_req_lch_1, acc_info_a_lch_1,);
criterion_main!(benches);
