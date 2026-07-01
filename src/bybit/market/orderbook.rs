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
use crate::bybit::exch_struct::{BYBIT, Exchange};
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

pub trait Orderbook<'a>: Exchange<'a> {
    fn orderbook_req(
        &'a self,
        symbol: &str,
        limit: usize,
    ) -> impl Future<Output = Result<RESULT_EXCH_BYBIT<RESULT_ORDERBOOK>, Error_req>>;
    fn orderbook(
        &'a self,
        symbol: &str,
        limit: usize,
    ) -> impl Future<Output = Result<RESULT_ORDERBOOK, Box<dyn std::error::Error>>> {
        async move { Ok(self.orderbook_req(symbol, limit).await?.result) }
    }

    fn orderbook_a(
        &'a self,
        symbol: &str,
        limit: usize,
    ) -> impl Future<Output = Result<RESULT_ORDERBOOK, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                async || self.orderbook(symbol, limit).await,
                usizezero(self.s().exch.timeout_cycle_ms),
            )
            .await
        }
    }

    fn orderbooks(
        &'a self,
        symbols: &'a [String],
        limit: usize,
    ) -> impl Future<Output = MAP<&'a str, Result<RESULT_ORDERBOOK, Box<dyn std::error::Error>>>>
    {
        async move {
            join_all(
                symbols
                    .iter()
                    .map(|v| async { (v.as_str(), self.orderbook(v.as_str(), limit).await) }),
            )
            .await
            .into_iter()
            .collect()
        }
    }

    fn orderbooks_a(
        &'a self,
        symbols: &'a [String],
        limit: usize,
    ) -> impl Future<Output = Result<MAP<&'a str, RESULT_ORDERBOOK>, Box<dyn Error>>> {
        async move {
            join_all(
                symbols.iter().map(|v| async {
                    Ok((v.as_str(), self.orderbook_a(v.as_str(), limit).await?))
                }),
            )
            .await
            .into_iter()
            .collect::<Result<_, Box<dyn Error>>>()
        }
    }
}

impl<'a> Orderbook<'a> for BYBIT<'a> {
    fn orderbook_req(
        &'a self,
        symbol: &str,
        limit: usize,
    ) -> impl Future<Output = Result<RESULT_EXCH_BYBIT<RESULT_ORDERBOOK>, Error_req>> {
        async move {
            Client::builder()
                .timeout(Duration::from_millis(
                    usizezero(self.s.exch.timeout_req_ms) as u64
                ))
                .build()?
                .get(format!(
                    "{}\
                {ORDERBOOK}\
                ?category={}\
                &symbol={symbol}\
                &limit={limit}",
                    &self.s.exch.url, &self.s.trade.category,
                ))
                .send()
                .await?
                .json()
                .await
        }
    }
}
