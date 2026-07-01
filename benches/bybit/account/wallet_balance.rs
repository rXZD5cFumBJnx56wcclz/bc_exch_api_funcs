#[path = "../prelude.rs"]
mod prelude;

use bc_exch_api_funcs::bybit::account::wallet_balance::*;
use prelude::*;

fn wallet_balance_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();

    c.bench_function("wallet_balance_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.wallet_balance_req("USDT"));
    });
}

fn wallet_balance_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();

    c.bench_function("wallet_balance_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.wallet_balance_a("USDT", 3));
    });
}

criterion_group!(benches, wallet_balance_req_lch_1, wallet_balance_a_lch_1,);
criterion_main!(benches);
