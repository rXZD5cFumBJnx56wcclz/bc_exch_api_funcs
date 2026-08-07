use std::{fmt::Debug, time::Duration};

use bc_utils_lg::structs::settings::SETTINGS_EXCH;

pub trait Exchange {}

#[derive(Debug)]
pub struct ResultWrap<T> {
    pub topic: Option<String>,
    pub time: Duration,
    pub res: T,
    pub info: Option<String>,
}
