#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde_with::{DisplayFromStr, serde_as};

use crate::exchs::bybit::prelude::*;
pub use crate::market::klines::*;

pub const KLINE: &str = "/v5/market/kline";

pub struct Klines {
    pub retry_or_timeout: RetryOrTimeout,
}

impl Klines {
    pub fn new(s: &SETTINGS_EXCH) -> Self {
        Self {
            retry_or_timeout: RetryOrTimeout {
                timeout: s.timeout_cycle_ms,
            },
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct WRAP_KLINE {
    pub symbol: String,
    pub category: String,
    #[serde_as(as = "Vec<Vec<DisplayFromStr>>")]
    pub list: Vec<Vec<f64>>,
}

fn run(
    cl: &Client,
    s: &SETTINGS_EXCH,
    symbol: &str,
    limit: usize,
    start: usize,
    end: usize,
) -> impl Future<Output = Result<Wrap<Vec<Vec<f64>>>, ExchangeError>> {
    async move {
        let req = cl
            .get(format!(
                "{}{KLINE}\
                        ?category={}\
                        &symbol={symbol}\
                        &interval={}\
                        &limit={limit}\
                        &start={start}\
                        &end={end}",
                s.url,
                s.category,
                s.timeframe_sec.as_secs() / 60,
            ))
            .send()
            .await
            .map_err(|e| ExchangeError::Http(e))?
            .json::<WRAP_REST<WRAP_KLINE>>()
            .await
            .map_err(|e| ExchangeError::Http(e))?;
        let mut klines = req.result.list;
        klines.reverse();
        Ok(Wrap {
            time: req.time,
            res: klines,
            info: Some(symbol.to_string()),
            topic: None,
        })
    }
}

impl KlinesTrait for Klines {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<Wrap<Vec<Vec<f64>>>, ExchangeError>> {
        async move {
            self.retry_or_timeout
                .run(|| run(cl, s, symbol, limit, start, end))
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::exchs::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn klines_res_1() {
        assert!(
            !Klines::new(&S)
                .run(&CL, &S, "SUIUSDT", 1000, 0, 0)
                .await
                .unwrap()
                .res
                .is_empty()
        );
    }
}
