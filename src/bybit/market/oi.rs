#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::prelude::*;
pub use crate::market::oi::*;
pub const OI: &str = "/v5/market/open-interest";

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct OI {
    pub symbol: String,
    pub category: String,
    pub list: Vec<OI1>,
}

impl ResultWrap<Vec<OI1>> for RESULT_EXCH_BYBIT<OI> {
    fn res(self) -> Vec<OI1> {
        self.result.list
    }
}

impl OpenInterest for BYBIT<'_> {
    fn oi_req<'a>(
        &'a self,
        symbol: &str,
        interval_time: &str,
        start_time: usize,
        end_time: usize,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<impl ResultWrap<Vec<OI1>>, Error_req>> {
        async move {
            self.client
                .get(format!(
                    "\
                        {}\
                        {OI}\
                        ?category={}\
                        &symbol={symbol}\
                        &intervalTime={interval_time}\
                        &startTime={start_time}\
                        &endTime={end_time}\
                        &limit={limit}\
                        &cursor={cursor}\
                    ",
                    &self.s().exch.url,
                    &self.s().trade.category,
                ))
                .send()
                .await?
                .json::<RESULT_EXCH_BYBIT<OI>>()
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bybit::market::oi::*;
    use crate::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn oi_req_lch_1() {
        println!("{:#?}", EXCH.oi_req("SUIUSDT", "5min", 0, 0, 1, "",).await);
    }

    #[tokio::test]
    async fn oi_a_lch_1() {
        println!(
            "{:#?}",
            EXCH.oi_a("SUIUSDT", "5min", 0, 0, 1, "",).await.unwrap()
        );
    }
}
