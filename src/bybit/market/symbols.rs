use std::error::Error;
use std::time::Duration;

use bc_utils_lg::structs::exch::bybit::result::RESULT_EXCH_BYBIT;
use bc_utils_lg::structs::exch::bybit::symbols::{
    RESULT_SYMBOLS, 
    RESULT_SYMBOLS1,
};
use reqwest::{
    Client,
    Error as Error_req
};
use bc_utils_core::mechanisms::all_or_nothing;

use crate::bybit::const_url::TICKERS;
use crate::deffunc::usizezero;


pub async fn symbols_req(
    api_url: &str,
    category: &str,
    symbol: &str,
    base_coin: &str,
    exp_date: &str,
    timeout_ms: &Duration,
) -> Result<RESULT_EXCH_BYBIT<RESULT_SYMBOLS>, Error_req>
{
    Client::builder()
        .timeout(*timeout_ms)
        .build()
        ?
        .get(
            format!(
                "{api_url}{TICKERS}\
                ?category={category}\
                &symbol={symbol}\
                &baseCoin={base_coin}\
                &expDate={exp_date}"
            )
        )
        .send()
        .await?
        .json::<RESULT_EXCH_BYBIT<RESULT_SYMBOLS>>()
    .await
}

pub async fn symbols(
    api_url: &str,
    category: &str,
    symbol: &str,
    base_coin: &str,
    exp_date: &str,
    timeout_ms: &usize,
) -> Result<Vec<RESULT_SYMBOLS1>, Box<dyn std::error::Error>>
{
    Ok(symbols_req(
        api_url, 
        category, 
        symbol, 
        base_coin, 
        exp_date,
        &Duration::from_millis(*usizezero(timeout_ms) as u64)
    ).await?.result.list)
}

pub async fn symbols_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    base_coin: &str,
    exp_date: &str,
    timeout_ms: &usize,
    timeout_cycle_ms: &usize,
) -> Result<Vec<RESULT_SYMBOLS1>, Box<dyn Error>>
{
    all_or_nothing(
        async || symbols(
            api_url, 
            category, 
            symbol, 
            base_coin, 
            exp_date,
            timeout_ms,
        )
            .await, 
        timeout_cycle_ms,
    ).await
}
