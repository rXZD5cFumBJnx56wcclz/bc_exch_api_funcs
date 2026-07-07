#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::bybit::const_url::ACC_INFO;
use crate::bybit::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RESULT_ACC_INFO {
    pub marginMode: String,
    pub updatedTime: String,
    pub unifiedMarginStatus: i32,
    pub dcpStatus: String,
    pub timeWindow: i32,
    pub smpGroup: i32,
    pub isMasterTrader: bool,
    pub spotHedgingStatus: String,
}

pub trait AccInfo: Exchange {
    fn acc_info_req<'a>(
        &'a self,
    ) -> impl Future<Output = Result<RESULT_EXCH_BYBIT<RESULT_ACC_INFO>, Error_req>>;
    fn acc_info<'a>(
        &'a self,
    ) -> impl Future<Output = Result<RESULT_ACC_INFO, Box<dyn std::error::Error>>> {
        async move { Ok(self.acc_info_req().await?.result) }
    }

    fn acc_info_a<'a>(&'a self) -> impl Future<Output = Result<RESULT_ACC_INFO, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                || self.acc_info(),
                usizezero(self.s().exch.timeout_cycle_ms),
            )
            .await
        }
    }
}
