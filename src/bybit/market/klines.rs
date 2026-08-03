#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde_with::{DisplayFromStr, serde_as};

use crate::bybit::prelude::*;
pub use crate::market::klines::*;

pub const KLINE: &str = "/v5/market/kline";

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct WRAP_KLINE {
    pub symbol: String,
    pub category: String,
    #[serde_as(as = "Vec<Vec<DisplayFromStr>>")]
    pub list: Vec<Vec<f64>>,
}

impl Klines for BYBIT {
    fn klines(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<ResultWrap<Vec<Vec<f64>>>, ExchangeError>> {
        async move {
            let req = self
                .rest_client
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
                    s.timeframe / 60_000,
                ))
                .send()
                .await
                .map_err(|e| ExchangeError::Http(e))?
                .json::<RESULT_EXCH_BYBIT<WRAP_KLINE>>()
                .await
                .map_err(|e| ExchangeError::Http(e))?;
            let mut klines = req.result.list;
            klines.reverse();
            Ok(ResultWrap {
                time: req.time,
                res: klines,
                info: Some(symbol.to_string()),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    #![warn(unused_must_use)]

    use crate::bybit::market::klines::*;

    use crate::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn klines_lch_1() {
        assert!(
            !EXCH()
                .klines(&S, "SUIUSDT", 1000, 0, 0)
                .await
                .unwrap()
                .res
                .is_empty()
        );
    }

    #[tokio::test]
    async fn klines_a_lch_1() {
        EXCH().klines_a(&S, "SUIUSDT", 10, 0, 0).await.unwrap();
    }

    #[tokio::test]
    async fn klines_a_res_1() {
        let res = EXCH()
            .klines_a(&S, "SUIUSDT", 1000, 0, 0)
            .await
            .unwrap()
            .res;
        if res[998][0] > res[999][0] || res.len() != 1000 {
            dbg!(res.len(), res[999][0], res[1000][0]);
            panic!();
        }
    }

    #[tokio::test]
    async fn klines_a_res_2() {
        let res = EXCH()
            .klines_a(&S, "BTCUSDT", 1000, 1669852800000, 1671062400000)
            .await
            .unwrap()
            .res;
        if res[998][0] > res[999][0] || res.len() != 1000 {
            dbg!(res.len(), res[998][0], res[999][0]);
            panic!();
        }
    }

    #[tokio::test]
    async fn klines_symbols_a_lch_1() {
        let symbols = vec![
            "SUIUSDT".to_string(),
            "ETHUSDT".to_string(),
            "ATOMUSDT".to_string(),
        ];
        EXCH()
            .klines_symbols_a(&S, symbols.as_slice(), 10, 0, 0)
            .await
            .unwrap();
    }
}
