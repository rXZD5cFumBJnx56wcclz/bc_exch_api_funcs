#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::error::Error;
use std::time::Duration;

use crate::bybit::result_req::RESULT_EXCH_BYBIT;
use bc_utils_core::mechanisms::all_or_nothing;
use reqwest::{Client, Error as Error_req};

use crate::bybit::const_url::TICKERS;
use crate::deffunc::usizezero;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_SYMBOLS1 {
    pub symbol: String,
    pub lastPrice: String,
    pub indexPrice: String,
    pub markPrice: String,
    pub prevPrice24h: String,
    pub price24hPcnt: String,
    pub highPrice24h: String,
    pub lowPrice24h: String,
    pub prevPrice1h: String,
    pub openInterest: String,
    pub openInterestValue: String,
    pub turnover24h: String,
    pub volume24h: String,
    pub fundingRate: String,
    pub nextFundingTime: String,
    pub predictedDeliveryPrice: String,
    pub basisRate: String,
    pub deliveryFeeRate: String,
    pub deliveryTime: String,
    pub ask1Size: String,
    pub bid1Price: String,
    pub ask1Price: String,
    pub bid1Size: String,
    pub basis: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_SYMBOLS {
    pub category: String,
    pub list: Vec<RESULT_SYMBOLS1>,
}

pub async fn symbols_req(
    api_url: &str,
    category: &str,
    symbol: &str,
    base_coin: &str,
    exp_date: &str,
    timeout_ms: &Duration,
) -> Result<RESULT_EXCH_BYBIT<RESULT_SYMBOLS>, Error_req> {
    Client::builder()
        .timeout(*timeout_ms)
        .build()?
        .get(format!(
            "{api_url}{TICKERS}\
                ?category={category}\
                &symbol={symbol}\
                &baseCoin={base_coin}\
                &expDate={exp_date}"
        ))
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
) -> Result<Vec<RESULT_SYMBOLS1>, Box<dyn std::error::Error>> {
    Ok(symbols_req(
        api_url,
        category,
        symbol,
        base_coin,
        exp_date,
        &Duration::from_millis(*usizezero(timeout_ms) as u64),
    )
    .await?
    .result
    .list)
}

pub async fn symbols_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    base_coin: &str,
    exp_date: &str,
    timeout_ms: &usize,
    timeout_cycle_ms: &usize,
) -> Result<Vec<RESULT_SYMBOLS1>, Box<dyn Error>> {
    all_or_nothing(
        async || symbols(api_url, category, symbol, base_coin, exp_date, timeout_ms).await,
        timeout_cycle_ms,
    )
    .await
}
