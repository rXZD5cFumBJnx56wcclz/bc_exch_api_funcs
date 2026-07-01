use std::sync::LazyLock;

use bc_utils_lg::settings::SETTINGS;
use bc_utils_lg::settings::settings_from_json;
use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::exch_struct::BYBIT;
use bc_exch_api_funcs::bybit::market::instr_info::*;

static S: LazyLock<SETTINGS> =
    LazyLock::new(|| settings_from_json("settings.json".into()).unwrap());
static EXCH: LazyLock<BYBIT<'_>> = LazyLock::new(|| BYBIT::new(&*S));

fn instr_info_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("instr_info_req_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.instr_info_req("", "", "", 1, ""));
    });
}

fn instr_info_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("instr_info_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.instr_info("BTCUSDT", "", ""));
    });
}

fn instr_info_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("instr_info_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.instr_info_a("", "", ""));
    });
}

fn instrs_info_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = &[
        "SUIUSDT".to_string(),
        "UNIUSDT".to_string(),
        "ETHUSDT".to_string(),
    ];
    c.bench_function("instrs_info_lch_1", |b| {
        b.to_async(&rtm).iter(|| EXCH.instrs_info(symbols, "", ""));
    });
}

fn instrs_info_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = &[
        "SUIUSDT".to_string(),
        "UNIUSDT".to_string(),
        "ETHUSDT".to_string(),
    ];
    c.bench_function("instrs_info_a_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| EXCH.instrs_info_a(symbols, "", ""));
    });
}

criterion_group!(
    benches,
    instr_info_req_lch_1,
    instr_info_lch_1,
    instr_info_a_lch_1,
    instrs_info_lch_1,
    instrs_info_a_lch_1,
);
criterion_main!(benches);
