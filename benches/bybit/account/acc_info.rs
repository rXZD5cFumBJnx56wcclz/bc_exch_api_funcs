use std::sync::LazyLock;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use tokio::runtime::Runtime;
use reqwest::Client;
use bc_utils_lg::funcs::settings::settings_from_json;
use bc_utils_lg::structs::settings::SETTINGS;

use bc_exch_api_funcs::bybit::account::acc_info::*;


static SETTINGS: LazyLock<SETTINGS> = LazyLock::new(|| {settings_from_json("./settings.json").unwrap()});

fn acc_info_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let cl = Client::new();
    c.bench_function("acc_info_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| acc_info_req(
            &cl,
            &SETTINGS.exch.key,
            &SETTINGS.exch.secret,
            &SETTINGS.exch.url,
        ));
    });
}

fn acc_info_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let cl = Client::new();
    c.bench_function("acc_info_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| acc_info_a(
            &cl,
            &SETTINGS.exch.key,
            &SETTINGS.exch.secret,
            &SETTINGS.exch.url,
            &3,
        ));
    });
}

criterion_group!(
    benches, 
    acc_info_req_lch_1,
    acc_info_a_lch_1,
);
criterion_main!(benches);