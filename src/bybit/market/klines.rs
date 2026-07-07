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
