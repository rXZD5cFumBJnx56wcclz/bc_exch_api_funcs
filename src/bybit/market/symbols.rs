#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{
    bybit::prelude::*,
    market::symbols::{SYMBOLS1, Symbols},
};
pub const TICKERS: &str = "/v5/market/tickers";

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct WRAP_SYMBOLS {
    pub category: String,
    pub list: Vec<SYMBOLS1>,
}

impl Symbols for BYBIT {
    fn symbols(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<ResultWrap<Vec<SYMBOLS1>>, ExchangeError>> {
        async move {
            let req = self
                .rest_client
                .get(format!(
                    "{}{TICKERS}\
                        ?category={}\
                        &symbol={symbol}\
                        &baseCoin={base_coin}\
                        &expDate={exp_date}",
                    &s.url, &s.category,
                ))
                .send()
                .await
                .map_err(|e| ExchangeError::Http(e))?
                .json::<RESULT_EXCH_BYBIT<WRAP_SYMBOLS>>()
                .await
                .map_err(|e| ExchangeError::Http(e))?;
            Ok(ResultWrap {
                time: req.time,
                res: req.result.list,
                info: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bybit::market::symbols::*;

    use crate::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn symbols_req_lch_1() {
        EXCH().symbols(&S, "", "", "").await.unwrap();
    }

    #[tokio::test]
    async fn symbols_a_lch_1() {
        EXCH().symbols_a(&S, "", "", "").await.unwrap();
    }

    #[tokio::test]
    async fn symbols_only_lch_1() {
        EXCH().symbols_only(&S,).await.unwrap();
    }
}
