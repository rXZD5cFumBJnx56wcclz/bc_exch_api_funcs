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

pub trait AccInfo<'a>: Exchange<'a> {
    fn acc_info_req(
        &'a self,
    ) -> impl Future<Output = Result<RESULT_EXCH_BYBIT<RESULT_ACC_INFO>, Error_req>>;
    fn acc_info(
        &'a self,
    ) -> impl Future<Output = Result<RESULT_ACC_INFO, Box<dyn std::error::Error>>> {
        async move { Ok(self.acc_info_req().await?.result) }
    }

    fn acc_info_a(&'a self) -> impl Future<Output = Result<RESULT_ACC_INFO, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                || self.acc_info(),
                usizezero(self.s().exch.timeout_cycle_ms),
            )
            .await
        }
    }
}
impl<'a> AccInfo<'a> for BYBIT<'a> {
    fn acc_info_req(
        &'a self,
    ) -> impl Future<Output = Result<RESULT_EXCH_BYBIT<RESULT_ACC_INFO>, Error_req>> {
        async move {
            let time_stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            self.client
                .get(format!("{}{}", &self.s.exch.url, ACC_INFO))
                .header(
                    "X-BAPI-SIGN",
                    hmac_(
                        self.s.exch.secret.as_bytes(),
                        format!("{}{}{}", time_stamp, &self.s.exch.key, 5000).as_bytes(),
                    ),
                )
                .header("X-BAPI-API-KEY", &self.s.exch.key)
                .header("X-BAPI-TIMESTAMP", time_stamp.to_string())
                .header("X-BAPI-RECV-WINDOW", 5000)
                .send()
                .await?
                .json()
                .await
        }
    }
}
