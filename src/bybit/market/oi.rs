#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::error::Error;
use std::time::Duration;

use bc_utils_core::mechanisms::all_or_nothing;
use reqwest::{Client, Error as Error_req};
use serde::{Deserialize, Serialize};

use crate::bybit::const_url::OI;
use crate::bybit::result_req::RESULT_EXCH_BYBIT;
use crate::deffunc::usizezero;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_OI1 {
    pub openInterest: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_OI {
    pub symbol: String,
    pub category: String,
    pub list: Vec<RESULT_OI1>,
}

pub async fn oi_req(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval_time: &str,
    start_time: &usize,
    end_time: &usize,
    limit: &usize,
    cursor: &str,
    timeout_ms: &Duration,
) -> Result<RESULT_EXCH_BYBIT<RESULT_OI>, Error_req> {
    Client::builder()
        .timeout(*timeout_ms)
        .build()?
        .get(format!(
            "\
            {api_url}\
            {OI}\
            ?category={category}\
            &symbol={symbol}\
            &intervalTime={interval_time}\
            &startTime={start_time}\
            &endTime={end_time}\
            &limit={limit}\
            &cursor={cursor}\
        "
        ))
        .send()
        .await?
        .json()
        .await
}

pub async fn oi(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval_time: &str,
    start_time: &usize,
    end_time: &usize,
    limit: &usize,
    cursor: &str,
    timeout_ms: &usize,
) -> Result<Vec<RESULT_OI1>, Box<dyn std::error::Error>> {
    Ok(oi_req(
        api_url,
        category,
        symbol,
        interval_time,
        start_time,
        end_time,
        limit,
        cursor,
        &Duration::from_millis(*usizezero(timeout_ms) as u64),
    )
    .await?
    .result
    .list)
}

pub async fn oi_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval_time: &str,
    start_time: &usize,
    end_time: &usize,
    limit: &usize,
    cursor: &str,
    timeout_ms: &usize,
    timeout_cycle_ms: &usize,
) -> Result<Vec<RESULT_OI1>, Box<dyn Error>> {
    all_or_nothing(
        || {
            oi(
                api_url,
                category,
                symbol,
                interval_time,
                start_time,
                end_time,
                limit,
                cursor,
                timeout_ms,
            )
        },
        timeout_cycle_ms,
    )
    .await
}
