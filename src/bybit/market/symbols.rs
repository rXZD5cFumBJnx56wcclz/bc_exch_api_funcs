#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{bybit::prelude::*, market::symbols::SYMBOLS1};
pub const TICKERS: &str = "/v5/market/tickers";

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct WRAP_SYMBOLS {
    pub category: String,
    pub list: Vec<SYMBOLS1>,
}

impl ResultWrap<Vec<SYMBOLS1>> for RESULT_EXCH_BYBIT<WRAP_SYMBOLS> {
    fn res(self) -> Vec<SYMBOLS1> {
        self.result.list
    }
}

pub trait Symbols: Exchange {
    fn symbols_req<'a>(
        &'a self,
        symbol: &str,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<impl ResultWrap<Vec<SYMBOLS1>>, Error_req>>;
    fn symbols<'a>(
        &'a self,
        symbol: &str,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<Vec<SYMBOLS1>, Box<dyn std::error::Error>>> {
        async move { Ok(self.symbols_req(symbol, base_coin, exp_date).await?.res()) }
    }

    fn symbols_a<'a>(
        &'a self,
        symbol: &str,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<Vec<SYMBOLS1>, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                async || self.symbols(symbol, base_coin, exp_date).await,
                usizezero(self.s().exch.timeout_cycle_ms),
            )
            .await
        }
    }
}

impl Symbols for BYBIT<'_> {
    fn symbols_req<'a>(
        &'a self,
        symbol: &str,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<impl ResultWrap<Vec<SYMBOLS1>>, Error_req>> {
        async move {
            self.client
                .get(format!(
                    "{}{TICKERS}\
                        ?category={}\
                        &symbol={symbol}\
                        &baseCoin={base_coin}\
                        &expDate={exp_date}",
                    &self.s.exch.url, &self.s.trade.category,
                ))
                .send()
                .await?
                .json::<RESULT_EXCH_BYBIT<WRAP_SYMBOLS>>()
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bybit::market::symbols::*;

    use crate::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn symbols_req_lch_1() {
        EXCH.symbols_req("", "", "").await.unwrap();
    }

    #[tokio::test]
    async fn symbols_a_lch_1() {
        EXCH.symbols_a("", "", "").await.unwrap();
    }
}
