#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bc_utils_core::mechanisms::{all_or_nothing, one_time_hm};

use bc_utils_lg::types::maps::MAP;
use futures::future::join_all;
use reqwest::{Client, Error as Error_req};
use serde::{Deserialize, Serialize};

use crate::bybit::const_url::KLINE;
use crate::bybit::result_req::RESULT_EXCH_BYBIT;
use crate::deffunc::usizezero;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_KLINE {
    pub symbol: String,
    pub category: String,
    pub list: Vec<Vec<String>>,
}

pub async fn klines_req(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval: &str,
    limit: usize,
    start: usize,
    end: usize,
    timeout: &Duration,
) -> Result<RESULT_EXCH_BYBIT<RESULT_KLINE>, Error_req> {
    Client::builder()
        .timeout(*timeout)
        .build()?
        .get(format!(
            "{api_url}{KLINE}\
                ?category={category}\
                &symbol={symbol}\
                &interval={interval}\
                &limit={limit}\
                &start={start}\
                &end={end}"
        ))
        .send()
        .await?
        .json::<RESULT_EXCH_BYBIT<RESULT_KLINE>>()
        .await
}

/// the function returns values from the beginning of the start to the end (in ascending order)
/// It's a cumbersome implementation, but I don't want to fuck with it right now.
pub async fn klines(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval: &str,
    limit: usize,
    start: usize,
    end: usize,
    timeout_ms: usize,
) -> Result<Vec<Vec<f64>>, Box<dyn std::error::Error>> {
    let inter = match interval {
        "D" => 1440,
        "W" => 10_080,
        "M" => 43_200,
        v => v.parse::<usize>().unwrap(),
    };
    let timeout_ms = Duration::from_millis(usizezero(timeout_ms) as u64);
    let time_stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as usize;
    let changes = inter * limit * 60 * 1000;
    let step = inter * 1000 * 60 * 1000;
    let mut klines = Vec::with_capacity(limit);
    let (start, end) = if start != 0 && end == 0 {
        (start, start + changes)
    } else if start == 0 && end != 0 {
        (end - changes, end)
    } else {
        (time_stamp - changes, time_stamp)
    };
    let mut futures = Vec::with_capacity(limit);
    for s in (start..end).step_by(step) {
        futures.push(async move {
            let mut res = klines_req(
                api_url,
                category,
                symbol,
                interval,
                limit,
                s,
                s + step,
                &timeout_ms,
            )
            .await?
            .result
            .list;
            res.reverse();
            Ok::<Vec<Vec<f64>>, Box<dyn Error>>(
                res.into_iter()
                    .map(|v| {
                        v.into_iter()
                            .map(|v1| v1.parse::<f64>().expect("this not a number"))
                            .collect()
                    })
                    .collect::<Vec<Vec<f64>>>(),
            )
        });
    }
    for v in join_all(futures).await {
        klines.extend_from_slice(&v?);
    }
    Ok(klines)
}

pub async fn klines_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval: &str,
    limit: usize,
    start: usize,
    end: usize,
    timeout_req_ms: usize,
    timeout_cycle_ms: usize,
) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    all_or_nothing(
        async || {
            klines(
                api_url,
                category,
                symbol,
                interval,
                limit,
                start,
                end,
                timeout_req_ms,
            )
            .await
        },
        timeout_cycle_ms,
    )
    .await
}

pub async fn kline_symbols<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    interval: &str,
    timeout_ms: usize,
) -> MAP<&'a str, Result<Vec<f64>, Box<dyn std::error::Error>>> {
    join_all(symbols.iter().map(|s| async {
        (
            s.as_str(),
            async {
                klines(api_url, category, s, interval, 1, 0, 0, timeout_ms)
                    .await?
                    .into_iter()
                    .next()
                    .ok_or(Box::from("not found"))
            }
            .await,
        )
    }))
    .await
    .into_iter()
    .collect()
}

pub async fn kline_symbols_a<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    interval: &str,
    timeout_req_ms: usize,
    timeout_cycle_ms: usize,
) -> Result<MAP<&'a str, Vec<f64>>, Box<dyn Error>> {
    join_all(symbols.iter().map(|s| async {
        Ok((
            s.as_str(),
            async {
                match klines_a(
                    api_url,
                    category,
                    s,
                    interval,
                    1,
                    0,
                    0,
                    timeout_req_ms,
                    timeout_cycle_ms,
                )
                .await
                {
                    Ok(v) => Ok(v.into_iter().next().unwrap_or(Default::default())),
                    Err(_) => Err(Box::<dyn Error>::from("klines_a err")),
                }
            }
            .await?,
        ))
    }))
    .await
    .into_iter()
    .collect::<Result<_, Box<dyn Error>>>()
}

pub async fn kline_symbols_ao<'a>(
    api_url: &'a str,
    category: &'a str,
    symbols: &'a [String],
    interval: &'a str,
    timeout_req_ms: usize,
    timeout_cycle_ms: usize,
) -> Result<MAP<&'a str, Vec<f64>>, Box<dyn Error>> {
    one_time_hm(
        async || {
            kline_symbols_a(
                api_url,
                category,
                symbols,
                interval,
                timeout_req_ms,
                timeout_cycle_ms,
            )
            .await
        },
        |v| Ok(v.1.get(0).ok_or(Box::<dyn Error>::from("err"))?),
        |v| {
            Ok(v.iter()
                .next()
                .ok_or(Box::<dyn Error>::from("err"))?
                .1
                .get(0)
                .ok_or(Box::<dyn Error>::from("err"))?)
        },
    )
    .await
}

pub async fn klines_symbols<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    interval: &str,
    limit: usize,
    start: usize,
    end: usize,
    timeout_req_ms: usize,
) -> MAP<&'a str, Result<Vec<Vec<f64>>, Box<dyn std::error::Error>>> {
    join_all(symbols.iter().map(|s| async {
        (
            s.as_str(),
            klines(
                api_url,
                category,
                s,
                interval,
                limit,
                start,
                end,
                timeout_req_ms,
            )
            .await,
        )
    }))
    .await
    .into_iter()
    .collect()
}

pub async fn klines_symbols_a<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    interval: &str,
    limit: usize,
    start: usize,
    end: usize,
    timeout_req_ms: usize,
    timeout_cycle_ms: usize,
) -> Result<MAP<&'a str, Vec<Vec<f64>>>, Box<dyn Error>> {
    join_all(symbols.iter().map(|s| async {
        Ok((
            s.as_str(),
            klines_a(
                api_url,
                category,
                s,
                interval,
                limit,
                start,
                end,
                timeout_req_ms,
                timeout_cycle_ms,
            )
            .await?,
        ))
    }))
    .await
    .into_iter()
    .collect::<Result<_, Box<dyn Error>>>()
}
