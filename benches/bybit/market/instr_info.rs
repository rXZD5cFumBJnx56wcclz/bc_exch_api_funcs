use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::market::instr_info::*;

fn instr_info_req_lch_1(c: &mut Criterion) {
    let dur = Duration::from_secs(3);
    let rtm = Runtime::new().unwrap();
    c.bench_function("instr_info_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| {
            instr_info_req(
                "https://api.bybit.com",
                "linear",
                "BTCUSDT",
                "",
                "",
                1,
                "",
                &dur,
            )
        });
    });
}

fn instr_info_lch_1(c: &mut Criterion) {
    let dur = 3;
    let rtm = Runtime::new().unwrap();
    c.bench_function("instr_info_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| instr_info("https://api.bybit.com", "linear", "BTCUSDT", "", "", dur));
    });
}

fn instr_info_a_lch_1(c: &mut Criterion) {
    let dur = 3;
    let rtm = Runtime::new().unwrap();
    c.bench_function("instr_info_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| {
            instr_info_a(
                "https://api.bybit.com",
                "linear",
                "BTCUSDT",
                "",
                "",
                dur,
                dur,
            )
        });
    });
}

fn instrs_info_lch_1(c: &mut Criterion) {
    let dur = 3;
    let rtm = Runtime::new().unwrap();
    let symbols = &[
        "SUIUSDT".to_string(),
        "UNIUSDT".to_string(),
        "ETHUSDT".to_string(),
    ];
    c.bench_function("instrs_info_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| instrs_info("https://api.bybit.com", "linear", symbols, "", "", dur));
    });
}

fn instrs_info_a_lch_1(c: &mut Criterion) {
    let dur = 3;
    let rtm = Runtime::new().unwrap();
    let symbols = &[
        "SUIUSDT".to_string(),
        "UNIUSDT".to_string(),
        "ETHUSDT".to_string(),
    ];
    c.bench_function("instrs_info_a_lch_1", |b| {
        b.to_async(&rtm)
            .iter(|| instrs_info_a("https://api.bybit.com", "linear", symbols, "", "", dur, dur));
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
