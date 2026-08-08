pub use std::sync::LazyLock;

pub use bc_exch_api_funcs::exchs::bybit::prelude::*;
pub use bc_utils_lg::structs::settings::SETTINGS_EXCH;
pub use bc_utils_lg::structs::settings::from_json;
pub use tokio;

pub static S: LazyLock<SETTINGS_EXCH> =
    LazyLock::new(|| from_json("settings.json".into()).unwrap());
pub static CL: LazyLock<Client> = LazyLock::new(|| new_rest_client(&S));
