use std::time::Duration;

use reqwest::Client;
use bc_utils_lg::structs::settings::SETTINGS;

use crate::deffunc::usizezero;
use crate::main_trait::Exchange;

#[derive(Debug, Clone)]
pub struct BYBIT<'a> {
    pub client: Client,
    pub s: &'a SETTINGS,
}

impl<'a> BYBIT<'a> {
    pub fn new(s: &'a SETTINGS) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_millis(
                    usizezero(s.exch.timeout_cycle_ms) as u64
                ))
                .build()
                .unwrap(),
            s,
        }
    }
}

impl Exchange for BYBIT<'_> {
    fn s<'a>(&'a self) -> &'a SETTINGS {
        &self.s
    }
}