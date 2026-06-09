#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::error::Error;
use std::time::Duration;

use bc_utils_core::mechanisms::all_or_nothing;

use bc_utils_lg::types::maps::MAP;
use futures::future::join_all;
use reqwest::{Client, Error as Error_req};
use serde::{Deserialize, Serialize};

use crate::bybit::const_url::ORDERBOOK;
use crate::bybit::result_req::RESULT_EXCH_BYBIT;
use crate::deffunc::usizezero;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_ORDERBOOK {
    pub s: String,
    pub a: Vec<Vec<String>>,
    pub b: Vec<Vec<String>>,
    pub ts: i64,
    pub u: i64,
    pub seq: i64,
    pub cts: i64,
}

pub async fn orderbook_req(
    api_url: &str,
    category: &str,
    symbol: &str,
    limit: usize,
    timeout_ms: &Duration,
) -> Result<RESULT_EXCH_BYBIT<RESULT_ORDERBOOK>, Error_req> {
    Client::builder()
        .timeout(*timeout_ms)
        .build()?
        .get(format!(
            "{api_url}\
            {ORDERBOOK}\
            ?category={category}\
            &symbol={symbol}\
        &limit={limit}"
        ))
        .send()
        .await?
        .json()
        .await
}

pub async fn orderbook(
    api_url: &str,
    category: &str,
    symbol: &str,
    limit: usize,
    timeout_ms: usize,
) -> Result<RESULT_ORDERBOOK, Box<dyn std::error::Error>> {
    Ok(orderbook_req(
        api_url,
        category,
        symbol,
        limit,
        &Duration::from_millis(usizezero(timeout_ms) as u64),
    )
    .await?
    .result)
}

pub async fn orderbook_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    limit: usize,
    timeout_ms: usize,
    timeout_cycle_ms: usize,
) -> Result<RESULT_ORDERBOOK, Box<dyn Error>> {
    all_or_nothing(
        async || orderbook(api_url, category, symbol, limit, timeout_ms).await,
        timeout_cycle_ms,
    )
    .await
}

pub async fn orderbooks<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    limit: usize,
    timeout_ms: usize,
) -> MAP<&'a str, Result<RESULT_ORDERBOOK, Box<dyn std::error::Error>>> {
    join_all(symbols.iter().map(|v| async {
        (
            v.as_str(),
            orderbook(api_url, category, v.as_str(), limit, timeout_ms).await,
        )
    }))
    .await
    .into_iter()
    .collect()
}

pub async fn orderbooks_a<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    limit: usize,
    timeout_ms: usize,
    timeout_cycle_ms: usize,
) -> Result<MAP<&'a str, RESULT_ORDERBOOK>, Box<dyn Error>> {
    join_all(symbols.iter().map(|v| async {
        Ok((
            v.as_str(),
            orderbook_a(
                api_url,
                category,
                v.as_str(),
                limit,
                timeout_ms,
                timeout_cycle_ms,
            )
            .await?,
        ))
    }))
    .await
    .into_iter()
    .collect::<Result<_, Box<dyn Error>>>()
}
