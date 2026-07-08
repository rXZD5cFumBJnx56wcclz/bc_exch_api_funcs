#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::prelude::*;
pub use crate::market::klines::*;

pub const KLINE: &str = "/v5/market/kline";

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct WRAP_KLINE {
    pub symbol: String,
    pub category: String,
    pub list: Vec<Vec<String>>,
}

impl ResultWrap<Vec<Vec<String>>> for RESULT_EXCH_BYBIT<WRAP_KLINE> {
    fn res(self) -> Vec<Vec<String>> {
        self.result.list
    }
}

impl Kline for BYBIT<'_> {
    fn klines_req<'a>(
        &'a self,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<impl ResultWrap<Vec<Vec<String>>>, Error_req>> {
        async move {
            self.client
                .get(format!(
                    "{}{KLINE}\
                        ?category={}\
                        &symbol={symbol}\
                        &interval={}\
                        &limit={limit}\
                        &start={start}\
                        &end={end}",
                    self.s().exch.url,
                    self.s().trade.category,
                    self.s().trade.timeframe,
                ))
                .send()
                .await?
                .json::<RESULT_EXCH_BYBIT<WRAP_KLINE>>()
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    #![warn(unused_must_use)]

    use crate::bybit::market::klines::*;

    use crate::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn klines_req_lch_1() {
        EXCH.klines_req("SUIUSDT", 100_000, 0, 0).await.unwrap();
    }

    #[tokio::test]
    async fn klines_a_lch_1() {
        EXCH.klines_a("SUIUSDT", 10, 0, 0).await.unwrap();
    }

    #[tokio::test]
    async fn klines_a_res_1() {
        let res = EXCH.klines_a("SUIUSDT", 99_999, 0, 0).await.unwrap();
        if res[999][0] > res[1000][0] || res.len() != 99_999 {
            dbg!(res.len(), res[999][0], res[1000][0]);
            panic!();
        }
    }

    #[tokio::test]
    async fn klines_a_res_2() {
        let res = EXCH
            .klines_a("BTCUSDT", 1100, 1669852800000, 1671062400000)
            .await
            .unwrap();
        if res[999][0] > res[1000][0] || res.len() != 1100 {
            dbg!(res.len(), res[999][0], res[1000][0]);
            panic!();
        }
    }

    #[tokio::test]
    async fn klines_a_res_3() {
        let res = EXCH
            .klines_a("BTCUSDT", 1100, 1669852800000, 1671062400000)
            .await
            .unwrap();
        if res[999][0] > res[1000][0] || res.len() != 1100 {
            dbg!(res.len(), res[999][0], res[1000][0]);
            panic!();
        }
    }

    #[tokio::test]
    async fn kline_symbols_lch_1() {
        let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
        let _ = EXCH.kline_symbols(symbols.as_slice()).await;
    }

    #[tokio::test]
    async fn kline_symbols_a_lch_1() {
        let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
        EXCH.kline_symbols_a(symbols.as_slice()).await.unwrap();
    }

    #[tokio::test]
    async fn kline_symbols_ao_lch_1() {
        let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
        EXCH.kline_symbols_ao(symbols.as_slice()).await.unwrap();
    }

    #[tokio::test]
    async fn klines_symbols_lch_1() {
        let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
        let _ = EXCH.klines_symbols(symbols.as_slice(), 10, 0, 0).await;
    }

    #[tokio::test]
    async fn klines_symbols_a_lch_1() {
        let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
        EXCH.klines_symbols_a(symbols.as_slice(), 10, 0, 0)
            .await
            .unwrap();
    }
}
