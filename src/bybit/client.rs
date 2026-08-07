use crate::bybit::prelude::*;

#[derive(Debug)]
pub struct RestClient {
    pub rest_client: Client,
}

impl RestClient {
    pub fn new(s: &SETTINGS_EXCH) -> Self {
        Self {
            rest_client: Client::builder()
                .timeout(s.timeout_cycle_ms)
                .build()
                .unwrap(),
        }
    }
}
