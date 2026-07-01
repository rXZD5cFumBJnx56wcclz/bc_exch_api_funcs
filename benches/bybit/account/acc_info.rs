use std::sync::LazyLock;

use bc_utils_lg::settings::SETTINGS;
use bc_utils_lg::settings::settings_from_json;
use criterion::{Criterion, criterion_group, criterion_main};
use reqwest::Client;
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::account::acc_info::*;
use bc_exch_api_funcs::bybit::exch_struct::BYBIT;

static S: LazyLock<SETTINGS> =
    LazyLock::new(|| settings_from_json("settings.json".into()).unwrap());
static EXCH: LazyLock<BYBIT<'_>> = LazyLock::new(|| BYBIT::new(&*S));

fn acc_info_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let cl = Client::new();
    c.bench_function("acc_info_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.acc_info_req(&cl));
    });
}

fn acc_info_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let cl = Client::new();
    c.bench_function("acc_info_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.acc_info_a(&cl));
    });
}

criterion_group!(benches, acc_info_req_lch_1, acc_info_a_lch_1,);
criterion_main!(benches);
