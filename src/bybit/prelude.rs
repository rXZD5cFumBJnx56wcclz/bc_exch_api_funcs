pub use std::error::Error;
pub use std::time::{Duration, SystemTime, UNIX_EPOCH};

// pub use bc_utils_core::hashing::hmac_;
// pub use bc_utils_core::mechanisms::{all_or_nothing, one_time_hm};
pub use bc_utils_lg::types::maps::MAP;
pub use futures::future::join_all;
pub use reqwest::{Client, Error as Error_req};
pub use serde::{Deserialize, Serialize};

pub use std::time::Instant;
// pub use std::time::UNIX_EPOCH;

pub use crate::bybit::exch_struct::*;
pub use crate::bybit::result_req::*;
pub use crate::prelude::*;
pub use crate::ws_connect::*;

pub const WS_HOST: &str = "stream.bybit.com";
pub const WS_PUBLIC: &str = "wss://stream.bybit.com/v5/public";
pub const WS_PRIVATE: &str = "wss://stream.bybit.com/v5/private";
