#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::prelude::*;
pub use crate::market::orderbook::*;
pub const ORDERBOOK: &str = "/v5/market/orderbook";

impl ResultWrap<ORDERBOOK> for RESULT_EXCH_BYBIT<ORDERBOOK> {
    fn res(self) -> ORDERBOOK {
        self.result
    }
}

impl Orderbook for BYBIT<'_> {
    fn orderbook_req<'a>(
        &'a self,
        symbol: &str,
        limit: usize,
    ) -> impl Future<Output = Result<impl ResultWrap<ORDERBOOK>, Error_req>> {
        async move {
            self.client
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
                .json::<RESULT_EXCH_BYBIT<ORDERBOOK>>()
                .await
        }
    }
}
