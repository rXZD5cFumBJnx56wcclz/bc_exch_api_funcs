use std::sync::LazyLock;

use bc_utils_lg::settings::SETTINGS;
use bc_utils_lg::settings::settings_from_json;
use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::exch_struct::BYBIT;
use bc_exch_api_funcs::bybit::market::symbols::*;

static S: LazyLock<SETTINGS> =
    LazyLock::new(|| settings_from_json("settings.json".into()).unwrap());
static EXCH: LazyLock<BYBIT<'_>> = LazyLock::new(|| BYBIT::new(&*S));

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
