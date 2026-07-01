#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::const_url::OI;
use crate::bybit::prelude::*;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_OI1 {
    pub openInterest: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_OI {
    pub symbol: String,
    pub category: String,
    pub list: Vec<RESULT_OI1>,
}

pub trait OpenInterest<'a>: Exchange<'a> {
    fn oi_req(
        &'a self,
        symbol: &str,
        interval_time: &str,
        start_time: usize,
        end_time: usize,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<RESULT_EXCH_BYBIT<RESULT_OI>, Error_req>>;
    fn oi(
        &'a self,
        symbol: &str,
        interval_time: &str,
        start_time: usize,
        end_time: usize,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<Vec<RESULT_OI1>, Box<dyn std::error::Error>>> {
        async move {
            Ok(self
                .oi_req(symbol, interval_time, start_time, end_time, limit, cursor)
                .await?
                .result
                .list)
        }
    }
    fn oi_a(
        &'a self,
        symbol: &str,
        interval_time: &str,
        start_time: usize,
        end_time: usize,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<Vec<RESULT_OI1>, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                || self.oi(symbol, interval_time, start_time, end_time, limit, cursor),
                usizezero(self.s().exch.timeout_cycle_ms),
            )
            .await
        }
    }
}

impl<'a> OpenInterest<'a> for BYBIT<'a> {
    fn oi_req(
        &'a self,
        symbol: &str,
        interval_time: &str,
        start_time: usize,
        end_time: usize,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<RESULT_EXCH_BYBIT<RESULT_OI>, Error_req>> {
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
                .json()
                .await
        }
    }
}
