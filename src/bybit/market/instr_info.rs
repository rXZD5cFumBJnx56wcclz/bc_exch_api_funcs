#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::error::Error;
use std::time::Duration;

use bc_utils_core::mechanisms::all_or_nothing;
use bc_utils_lg::types::maps::MAP;
use reqwest::{Client, Error as Error_req};
use serde::{Deserialize, Serialize};

use crate::bybit::const_url::INSTR_INFO;
use crate::bybit::result_req::RESULT_EXCH_BYBIT;
use crate::deffunc::usizezero;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_INSTR_INFO2_LEVERAGE_FILTER {
    pub minLeverage: String,
    pub maxLeverage: String,
    pub leverageStep: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_INSTR_INFO2_PRICE_FILTER {
    pub minPrice: String,
    pub maxPrice: String,
    pub tickSize: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_INSTR_INFO2_LOT_SIZE_FILTER {
    pub maxOrderQty: String,
    pub minOrderQty: String,
    pub qtyStep: String,
    pub postOnlyMaxOrderQty: String,
    pub maxMktOrderQty: String,
    pub minNotionalValue: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_INSTR_INFO2_RISK_PARAMETERS {
    pub priceLimitRatioX: String,
    pub priceLimitRatioY: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_INSTR_INFO1 {
    symbol: String,
    pub contractType: String,
    status: String,
    pub baseCoin: String,
    pub quoteCoin: String,
    pub launchTime: String,
    pub deliveryTime: String,
    pub deliveryFeeRate: String,
    pub priceScale: String,
    pub leverageFilter: RESULT_INSTR_INFO2_LEVERAGE_FILTER,
    pub priceFilter: RESULT_INSTR_INFO2_PRICE_FILTER,
    pub lotSizeFilter: RESULT_INSTR_INFO2_LOT_SIZE_FILTER,
    pub unifiedMarginTrade: bool,
    pub fundingInterval: i32,
    settleCoin: String,
    pub copyTrading: String,
    pub upperFundingRate: String,
    pub lowerFundingRate: String,
    pub isPreListing: bool,
    pub preListingInfo: Option<String>,
    pub riskParameters: RESULT_INSTR_INFO2_RISK_PARAMETERS,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_INSTR_INFO {
    pub category: String,
    pub list: Vec<RESULT_INSTR_INFO1>,
    pub nextPageCursor: String,
}

pub async fn instr_info_req(
    api_url: &str,
    category: &str,
    symbol: &str,
    status: &str,
    base_coin: &str,
    limit: usize,
    cursor: &str,
    timeout_ms: &Duration,
) -> Result<RESULT_EXCH_BYBIT<RESULT_INSTR_INFO>, Error_req> {
    Client::builder()
        .timeout(*timeout_ms)
        .build()?
        .get(format!(
            "{api_url}{INSTR_INFO}\
            ?category={category}\
            &symbol={symbol}\
            &status={status}\
            &baseCoin={base_coin}\
            &limit={limit}\
            &cursor={cursor}"
        ))
        .send()
        .await?
        .json::<RESULT_EXCH_BYBIT<RESULT_INSTR_INFO>>()
        .await
}

pub async fn instr_info(
    api_url: &str,
    category: &str,
    symbol: &str,
    status: &str,
    base_coin: &str,
    timeout_ms: usize,
) -> Result<RESULT_INSTR_INFO1, Box<dyn std::error::Error>> {
    instr_info_req(
        api_url,
        category,
        symbol,
        status,
        base_coin,
        1,
        "",
        &Duration::from_millis(usizezero(timeout_ms) as u64),
    )
    .await?
    .result
    .list
    .into_iter()
    .next()
    .ok_or(Box::from("not found"))
}

pub async fn instr_info_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    status: &str,
    base_coin: &str,
    timeout_ms: usize,
    timeout_cycle_ms: usize,
) -> Result<RESULT_INSTR_INFO1, Box<dyn Error>> {
    all_or_nothing(
        async || instr_info(api_url, category, symbol, status, base_coin, timeout_ms).await,
        timeout_cycle_ms,
    )
    .await
}

pub async fn instrs_info<'a>(
    api_url: &'a str,
    category: &'a str,
    symbols: &'a [String],
    status: &'a str,
    base_coin: &'a str,
    timeout_ms: usize,
) -> Result<MAP<&'a str, RESULT_INSTR_INFO1>, Box<dyn std::error::Error>> {
    let timeout_ms = Duration::from_millis(usizezero(timeout_ms) as u64);
    let mut res = MAP::default();
    let mut passed = vec![];
    let mut cursor = "".to_string();
    while passed.len() != symbols.len() {
        let response_ = instr_info_req(
            api_url,
            category,
            "",
            status,
            base_coin,
            // fix this `limit` arg ↓
            1000,
            &cursor,
            &timeout_ms,
        )
        .await?
        .result;
        cursor = response_.nextPageCursor.clone();
        for v in response_.list.into_iter() {
            for s in symbols {
                if s == &v.symbol {
                    res.insert(s.as_str(), v);
                    passed.push(s.as_str());
                    break;
                }
            }
        }
    }
    Ok(res)
}

pub async fn instrs_info_a<'a>(
    api_url: &'a str,
    category: &'a str,
    symbols: &'a [String],
    status: &'a str,
    base_coin: &'a str,
    timeout_ms: usize,
    timeout_cycle_ms: usize,
) -> Result<MAP<&'a str, RESULT_INSTR_INFO1>, Box<dyn Error>> {
    all_or_nothing(
        || instrs_info(api_url, category, symbols, status, base_coin, timeout_ms),
        timeout_cycle_ms,
    )
    .await
}
