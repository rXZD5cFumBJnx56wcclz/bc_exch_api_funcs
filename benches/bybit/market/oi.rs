use std::sync::LazyLock;

use bc_utils_lg::settings::SETTINGS;
use bc_utils_lg::settings::settings_from_json;
use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::exch_struct::BYBIT;
use bc_exch_api_funcs::bybit::market::oi::*;

static S: LazyLock<SETTINGS> =
    LazyLock::new(|| settings_from_json("settings.json".into()).unwrap());
static EXCH: LazyLock<BYBIT<'_>> = LazyLock::new(|| BYBIT::new(&*S));

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
