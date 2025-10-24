#![allow(non_camel_case_types)]

use std::error::Error;
use std::time::Duration;

use reqwest::{
    Error as Error_req,
    Client,
};
use futures::future::join_all;
use bc_utils_lg::structs::exch::bybit::result::RESULT_EXCH_BYBIT;
use bc_utils_lg::structs::exch::bybit::orderbook::RESULT_ORDERBOOK;
use bc_utils_lg::types::maps::MAP;
use bc_utils_core::mechanisms::all_or_nothing;

use crate::bybit::const_url::ORDERBOOK;
use crate::deffunc::usizezero;

pub async fn orderbook_req(
    api_url: &str,
    category: &str,
    symbol: &str,
    limit: &usize,
    timeout_ms: &Duration
) -> Result<RESULT_EXCH_BYBIT<RESULT_ORDERBOOK>, Error_req>
{
    Client::builder()
        .timeout(*timeout_ms)
        .build()
        ?
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
    limit: &usize,
    timeout_ms: &usize,
) -> Result<RESULT_ORDERBOOK, Box<dyn std::error::Error>>
{
    Ok(orderbook_req(
        api_url, 
        category, 
        symbol,
        limit,
        &Duration::from_millis(*usizezero(timeout_ms) as u64),
    ).await?.result)
}

pub async fn orderbook_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    limit: &usize,
    timeout_ms: &usize,
    timeout_cycle_ms: &usize,
) -> Result<RESULT_ORDERBOOK, Box<dyn Error>>
{
    all_or_nothing(async || orderbook(
        api_url, 
        category, 
        symbol, 
        limit,
        timeout_ms,
    ).await, timeout_cycle_ms).await
}

pub async fn orderbooks<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    limit: &usize,
    timeout_ms: &usize,
) -> MAP<&'a str, Result<RESULT_ORDERBOOK, Box<dyn std::error::Error>>>
{
    join_all(
        symbols
            .iter()
            .map(
                |v| async {
                    (
                        v.as_str(), 
                        orderbook(
                            api_url, 
                            category, 
                            v.as_str(), 
                            limit,
                            timeout_ms,
                        ).await
                    )
                }
            )
    )
        .await
        .into_iter()
        .collect()
}

pub async fn orderbooks_a<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    limit: &usize,
    timeout_ms: &usize,
    timeout_cycle_ms: &usize,
) -> Result<MAP<&'a str, RESULT_ORDERBOOK>, Box<dyn Error>>
{
    join_all(
        symbols
            .iter()
            .map(
                |v| async {
                    Ok((
                        v.as_str(), 
                        orderbook_a(
                            api_url, 
                            category, 
                            v.as_str(), 
                            limit,
                            timeout_ms,
                            timeout_cycle_ms,
                        ).await?
                    ))
                }
            )
    )
        .await
        .into_iter()
        .collect::<Result<_, Box<dyn Error>>>()
}
