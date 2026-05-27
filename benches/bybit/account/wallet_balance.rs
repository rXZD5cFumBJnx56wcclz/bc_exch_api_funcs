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

use bc_exch_api_funcs::bybit::account::wallet_balance::*;


static SETTINGS: LazyLock<SETTINGS> = LazyLock::new(|| {settings_from_json("./settings.json").unwrap()});

fn wallet_balance_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let cl = Client::new();
    c.bench_function("wallet_balance_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| wallet_balance_req(
            &cl,
            &SETTINGS.exch.key,
            &SETTINGS.exch.secret,
            &SETTINGS.exch.url,
            "UNIFIED",
            "USDT",
        ));
    });
}

fn wallet_balance_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let cl = Client::new();
    c.bench_function("wallet_balance_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| wallet_balance_a(
            &cl,
            &SETTINGS.exch.key,
            &SETTINGS.exch.secret,
            &SETTINGS.exch.url,
            "UNIFIED",
            "USDT",
            &3,
        ));
    });
}

criterion_group!(
    benches, 
    wallet_balance_req_lch_1,
    wallet_balance_a_lch_1,
);
criterion_main!(benches);