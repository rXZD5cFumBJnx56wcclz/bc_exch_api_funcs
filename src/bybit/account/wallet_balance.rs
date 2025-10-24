use std::time::{
    SystemTime, 
    UNIX_EPOCH,
};
use std::error::Error;

use bc_utils_lg::structs::exch::bybit::result::RESULT_EXCH_BYBIT;
use bc_utils_lg::structs::exch::bybit::wallet_balance::{
    RESULT_WALLET_BALANCE, 
    RESULT_WALLET_BALANCE1
};
use reqwest::{
    Client, 
    Error as Error_req
};
use bc_utils_core::mechanisms::all_or_nothing;
use bc_utils_core::hashing::hmac_;

use crate::bybit::const_url::WALLET_BALANCE;


pub async fn wallet_balance_req(
    client: &Client,
    token: &str,
    secr: &str,
    api_url: &str,
    account_type: &str,
    coin: &str,
) -> Result<RESULT_EXCH_BYBIT<RESULT_WALLET_BALANCE>, Error_req>
{
    let time_stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let query = format!("accountType={account_type}&coin={coin}");
    client
        .get(format!("{}{}?{}", api_url, WALLET_BALANCE, query))
        .header("X-BAPI-SIGN", hmac_(
            secr.as_bytes(), 
            format!("{}{}{}{}", time_stamp, token, 5000, query).as_bytes()
        ))
        .header("X-BAPI-API-KEY", token)
        .header("X-BAPI-TIMESTAMP", time_stamp.to_string())
        .header("X-BAPI-RECV-WINDOW", 5000)
        .send()
        .await?
        .json()
        .await
}

pub async fn wallet_balance(
    client: &Client,
    token: &str,
    secr: &str,
    api_url: &str,
    account_type: &str,
    coin: &str,
) -> Result<Vec<RESULT_WALLET_BALANCE1>, Box<dyn std::error::Error>>
{
    Ok(wallet_balance_req(
        client, 
        token, 
        secr, 
        api_url, 
        account_type, 
        coin,
    ).await?.result.list)
}

pub async fn wallet_balance_a(
    client: &Client,
    token: &str,
    secr: &str,
    api_url: &str,
    account_type: &str,
    coin: &str,
    timeout_cycle_ms: &usize,
) -> Result<Vec<RESULT_WALLET_BALANCE1>, Box<dyn Error>>
{
    all_or_nothing(|| wallet_balance(
        client, 
        token, 
        secr, 
        api_url, 
        account_type, 
        coin,
    ), timeout_cycle_ms).await
}
