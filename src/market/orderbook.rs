#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::prelude::*;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct ORDERBOOK {
    pub s: String,
    pub a: Vec<Vec<String>>,
    pub b: Vec<Vec<String>>,
    pub ts: i64,
    pub u: i64,
    pub seq: i64,
    pub cts: i64,
}

pub trait Orderbook {
    fn orderbook_req(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        limit: usize,
    ) -> impl Future<Output = Result<ResultWrap<ORDERBOOK>, Error_req>>;
    fn orderbook(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        limit: usize,
    ) -> impl Future<Output = Result<ORDERBOOK, Box<dyn std::error::Error>>> {
        async move { Ok(self.orderbook_req(s, symbol, limit).await?.res()) }
    }

    fn orderbook_a(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        limit: usize,
    ) -> impl Future<Output = Result<ORDERBOOK, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                async || self.orderbook(s, symbol, limit).await,
                usizezero(s.timeout_cycle_ms),
            )
            .await
        }
    }

    fn orderbooks<'a>(
        &self,
        s: &SETTINGS_EXCH,
        symbols: &'a [String],
        limit: usize,
    ) -> impl Future<Output = MAP<&'a str, Result<ORDERBOOK, Box<dyn std::error::Error>>>> {
        async move {
            join_all(
                symbols
                    .iter()
                    .map(|v| async { (v.as_str(), self.orderbook(s, v.as_str(), limit).await) }),
            )
            .await
            .into_iter()
            .collect()
        }
    }

    fn orderbooks_a<'a>(
        &self,
        s: &SETTINGS_EXCH,
        symbols: &'a [String],
        limit: usize,
    ) -> impl Future<Output = Result<MAP<&'a str, ORDERBOOK>, Box<dyn Error>>> {
        async move {
            join_all(
                symbols.iter().map(|v| async {
                    Ok((v.as_str(), self.orderbook_a(s, v.as_str(), limit).await?))
                }),
            )
            .await
            .into_iter()
            .collect::<Result<_, Box<dyn Error>>>()
        }
    }
}
