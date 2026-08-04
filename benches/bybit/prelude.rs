pub use std::sync::LazyLock;
pub use std::time::Instant;

pub use bc_exch_api_funcs::benches::*;
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
