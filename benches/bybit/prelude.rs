pub use std::sync::LazyLock;
pub use std::time::Instant;

pub use bc_exch_api_funcs::prelude::*;
pub use bc_utils_lg::structs::settings::SETTINGS_EXCH;
pub use bc_utils_lg::structs::settings::from_json;
pub use criterion::{Criterion, criterion_group, criterion_main};
pub use tokio;
pub use tokio::runtime::Runtime;

pub use bc_exch_api_funcs::bybit::exch_struct::BYBIT;

pub static S: LazyLock<SETTINGS_EXCH> =
    LazyLock::new(|| from_json("settings.json".into()).unwrap());
pub static EXCH: LazyLock<fn() -> BYBIT> = LazyLock::new(|| || BYBIT::new_rest(&*S));

pub fn stat_bench(res: &[Duration]) -> String {
    let mut sorted = res.to_vec();
    sorted.sort();
    format!(
        "\
        secs:\n\n\
        avg: {}\n\
        median: {}\n\
        min: {}\n\
        max: {}\n\
        ",
        sorted.iter().map(|v| v.as_secs_f64()).sum::<f64>() / sorted.len() as f64,
        sorted[sorted.len() / 2].as_secs_f64(),
        sorted.iter().min().unwrap().as_secs_f64(),
        sorted.iter().max().unwrap().as_secs_f64(),
    )
}
