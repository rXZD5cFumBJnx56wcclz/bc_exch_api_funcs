pub use std::sync::LazyLock;

pub use bc_utils_lg::structs::settings::SETTINGS;
pub use bc_utils_lg::structs::settings::settings_from_json;
pub use criterion::{Criterion, criterion_group, criterion_main};
pub use reqwest::Client;
pub use tokio;
pub use tokio::runtime::Runtime;

pub use bc_exch_api_funcs::bybit::exch_struct::BYBIT;

pub static S: LazyLock<SETTINGS> =
    LazyLock::new(|| settings_from_json("settings.json".into()).unwrap());
pub static EXCH: LazyLock<BYBIT<'_>> = LazyLock::new(|| BYBIT::new(&*S));
