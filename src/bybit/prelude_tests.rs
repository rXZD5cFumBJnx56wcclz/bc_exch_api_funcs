#![allow(unused_imports)]
#[cfg(test)]
pub mod prelude {
    pub use std::sync::LazyLock;

    pub use bc_utils_lg::structs::settings::SETTINGS;
    pub use bc_utils_lg::structs::settings::SETTINGS_EXCH;
    pub use bc_utils_lg::structs::settings::from_json;
    pub use pretty_assertions::assert_eq as assert_eq_pr;
    pub use reqwest::Client;
    pub use tokio;

    pub use crate::bybit::exch_struct::BYBIT;

    pub static S: LazyLock<SETTINGS_EXCH> =
        LazyLock::new(|| from_json("settings.json".into()).unwrap());
    pub static EXCH: LazyLock<fn() -> BYBIT> = LazyLock::new(|| || BYBIT::new_rest(&*S));
}
