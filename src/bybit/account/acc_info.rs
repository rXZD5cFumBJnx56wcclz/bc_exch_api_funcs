#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use bc_utils_core::hashing::hmac_;
use bc_utils_core::mechanisms::all_or_nothing;
use reqwest::{Client, Error as Error_req};
use serde::{Deserialize, Serialize};

use crate::bybit::const_url::ACC_INFO;
use crate::bybit::result_req::RESULT_EXCH_BYBIT;

#[derive(Serialize, Deserialize, Debug)]
pub struct RESULT_ACC_INFO {
    pub marginMode: String,
    pub updatedTime: String,
    pub unifiedMarginStatus: i32,
    pub dcpStatus: String,
    pub timeWindow: i32,
    pub smpGroup: i32,
    pub isMasterTrader: bool,
    pub spotHedgingStatus: String,
}

pub async fn acc_info_req(
    client: &Client,
    token: &str,
    secr: &str,
    api_url: &str,
) -> Result<RESULT_EXCH_BYBIT<RESULT_ACC_INFO>, Error_req> {
    let time_stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    client
        .get(format!("{}{}", api_url, ACC_INFO))
        .header(
            "X-BAPI-SIGN",
            hmac_(
                secr.as_bytes(),
                format!("{}{}{}", time_stamp, token, 5000).as_bytes(),
            ),
        )
        .header("X-BAPI-API-KEY", token)
        .header("X-BAPI-TIMESTAMP", time_stamp.to_string())
        .header("X-BAPI-RECV-WINDOW", 5000)
        .send()
        .await?
        .json()
        .await
}

pub async fn acc_info(
    client: &Client,
    token: &str,
    secr: &str,
    api_url: &str,
) -> Result<RESULT_ACC_INFO, Box<dyn std::error::Error>> {
    Ok(acc_info_req(client, token, secr, api_url).await?.result)
}

pub async fn acc_info_a(
    client: &Client,
    token: &str,
    secr: &str,
    api_url: &str,
    timeout_cycle_ms: &usize,
) -> Result<RESULT_ACC_INFO, Box<dyn Error>> {
    all_or_nothing(|| acc_info(client, token, secr, api_url), timeout_cycle_ms).await
}
