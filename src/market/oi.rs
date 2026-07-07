#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::prelude::*;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct OI1 {
    pub openInterest: String,
    pub timestamp: String,
}

pub trait OpenInterest: Exchange {
    fn oi_req<'a>(
        &'a self,
        symbol: &str,
        interval_time: &str,
        start_time: usize,
        end_time: usize,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<impl ResultWrap<Vec<OI1>>, Error_req>>;
    fn oi<'a>(
        &'a self,
        symbol: &str,
        interval_time: &str,
        start_time: usize,
        end_time: usize,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<Vec<OI1>, Box<dyn std::error::Error>>> {
        async move {
            Ok(self
                .oi_req(symbol, interval_time, start_time, end_time, limit, cursor)
                .await?
                .res())
        }
    }
    fn oi_a<'a>(
        &'a self,
        symbol: &str,
        interval_time: &str,
        start_time: usize,
        end_time: usize,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<Vec<OI1>, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                || self.oi(symbol, interval_time, start_time, end_time, limit, cursor),
                usizezero(self.s().exch.timeout_cycle_ms),
            )
            .await
        }
    }
}
