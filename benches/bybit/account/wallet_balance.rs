use std::sync::LazyLock;

use bc_utils_lg::settings::SETTINGS;
use bc_utils_lg::settings::settings_from_json;
use criterion::{Criterion, criterion_group, criterion_main};
use reqwest::Client;
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::account::wallet_balance::*;
use bc_exch_api_funcs::bybit::exch_struct::BYBIT;

static S: LazyLock<SETTINGS> =
    LazyLock::new(|| settings_from_json("settings.json".into()).unwrap());
static EXCH: LazyLock<BYBIT<'_>> = LazyLock::new(|| BYBIT::new(&*S));

fn wallet_balance_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let cl = Client::new();
    c.bench_function("wallet_balance_req_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.wallet_balance_req(&cl, "UNIFIED", "USDT"));
    });
}

fn wallet_balance_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let cl = Client::new();
    c.bench_function("wallet_balance_a_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.wallet_balance_a(&cl, "UNIFIED", "USDT", 3));
    });
}

criterion_group!(benches, wallet_balance_req_lch_1, wallet_balance_a_lch_1,);
criterion_main!(benches);
