#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::prelude::*;
pub use crate::market::orderbook::*;
pub const ORDERBOOK: &str = "/v5/market/orderbook";

ResultWrap<ORDERBOOK> for RESULT_EXCH_BYBIT<ORDERBOOK> {
    fn res(self) -> ORDERBOOK {
        self.result
    }
}

impl Orderbook for BYBIT {
    fn orderbook_req(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        limit: usize,
    ) -> impl Future<Output = Result<ResultWrap<ORDERBOOK>, Error_req>> {
        async move {
            self.client
                .get(format!(
                    "{}\
                {ORDERBOOK}\
                ?category={}\
                &symbol={symbol}\
                &limit={limit}",
                    &s.url, &s.category,
                ))
                .send()
                .await?
                .json::<RESULT_EXCH_BYBIT<ORDERBOOK>>()
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bybit::market::orderbook::*;
    use crate::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn orderbook_req_lch_1() {
        println!(
            "{:#?}",
            EXCH().orderbook_req(&S, "SUIUSDT", 10,).await.unwrap()
        );
    }

    #[tokio::test]
    async fn orderbook_a_lch_1() {
        EXCH().orderbook_a(&S, "SUIUSDT", 10).await.unwrap();
    }

    #[tokio::test]
    async fn orderbooks_lch_1() {
        let symbols = &[
            "SUIUSDT".to_string(),
            "WALRUSUSDT".to_string(),
            "ATOMUSDT".to_string(),
        ];
        let _ = EXCH().orderbooks(&S, symbols, 10).await;
    }

    #[tokio::test]
    async fn orderbooks_a_lch_1() {
        let symbols = &[
            "SUIUSDT".to_string(),
            "ETHUSDT".to_string(),
            "ATOMUSDT".to_string(),
        ];
        EXCH().orderbooks_a(&S, symbols, 10).await.unwrap();
    }
}
