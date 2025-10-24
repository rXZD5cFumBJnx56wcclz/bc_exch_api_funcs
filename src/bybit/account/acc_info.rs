use std::time::{
    SystemTime, 
    UNIX_EPOCH, 
};
use std::error::Error;

use bc_utils_lg::structs::exch::bybit::result::RESULT_EXCH_BYBIT;
use bc_utils_lg::structs::exch::bybit::acc_info::RESULT_ACC_INFO;
use reqwest::{
    Client, 
    Error as Error_req
};
use bc_utils_core::mechanisms::all_or_nothing;
use bc_utils_core::hashing::hmac_;

use crate::bybit::const_url::ACC_INFO;


pub async fn acc_info_req(
    client: &Client,
    token: &str,
    secr: &str,
    api_url: &str,
) -> Result<RESULT_EXCH_BYBIT<RESULT_ACC_INFO>, Error_req>
{
    let time_stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    client
        .get(format!("{}{}", api_url, ACC_INFO))
        .header("X-BAPI-SIGN", hmac_(
            secr.as_bytes(), 
            format!("{}{}{}", time_stamp, token, 5000).as_bytes()
        ))
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
) -> Result<RESULT_ACC_INFO, Box<dyn std::error::Error>>
{
    Ok(acc_info_req(
        client, 
        token, 
        secr, 
        api_url,
    ).await?.result)
}

pub async fn acc_info_a(
    client: &Client,
    token: &str,
    secr: &str,
    api_url: &str,
    timeout_cycle_ms: &usize,
) -> Result<RESULT_ACC_INFO, Box<dyn Error>>
{
    all_or_nothing(|| acc_info(client, token, secr, api_url,), timeout_cycle_ms).await
}
