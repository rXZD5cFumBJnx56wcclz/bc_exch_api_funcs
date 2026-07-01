#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use bc_utils_core::hashing::hmac_;
use bc_utils_core::mechanisms::all_or_nothing;
use reqwest::{Client, Error as Error_req};
use serde::{Deserialize, Serialize};

use crate::bybit::const_url::ACC_INFO;
use crate::bybit::exch_struct::{BYBIT, Exchange};
use crate::bybit::result_req::RESULT_EXCH_BYBIT;
use crate::deffunc::usizezero;

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
        client: &Client,
    ) -> impl Future<Output = Result<RESULT_EXCH_BYBIT<RESULT_ACC_INFO>, Error_req>>;
    fn acc_info(
        &'a self,
        client: &Client,
    ) -> impl Future<Output = Result<RESULT_ACC_INFO, Box<dyn std::error::Error>>> {
        async move { Ok(self.acc_info_req(client).await?.result) }
    }

    fn acc_info_a(
        &'a self,
        client: &Client,
    ) -> impl Future<Output = Result<RESULT_ACC_INFO, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                || self.acc_info(client),
                usizezero(self.s().exch.timeout_cycle_ms),
            )
            .await
        }
    }
}
impl<'a> AccInfo<'a> for BYBIT<'a> {
    fn acc_info_req(
        &'a self,
        client: &Client,
    ) -> impl Future<Output = Result<RESULT_EXCH_BYBIT<RESULT_ACC_INFO>, Error_req>> {
        async move {
            let time_stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            client
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
