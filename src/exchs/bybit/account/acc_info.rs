#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub use crate::account::acc_info::*;
use crate::exchs::bybit::prelude::*;
pub const ACC_INFO: &str = "/v5/account/info";

pub struct AccInfo {
    pub retry_or_timeout: RetryOrTimeout,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ACC_INFO {
    pub marginMode: String,
    pub updatedTime: String,
    pub unifiedMarginStatus: i32,
    pub dcpStatus: String,
    pub timeWindow: i32,
    pub smpGroup: i32,
    pub isMasterTrader: bool,
    pub spotHedgingStatus: String,
}

fn run(
    cl: &Client,
    s: &SETTINGS_EXCH,
) -> impl Future<Output = Result<ResultWrap<ACC_INFO>, ExchangeError>> {
    async move {
        let time_stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let res = cl
            .get(format!("{}{}", &s.url, ACC_INFO))
            .header(
                "X-BAPI-SIGN",
                hmac_(
                    s.secret.as_bytes(),
                    format!("{}{}{}", time_stamp, &s.key, s.timeout_req_ms.as_millis()).as_bytes(),
                ),
            )
            .header("X-BAPI-API-KEY", &s.key)
            .header("X-BAPI-TIMESTAMP", time_stamp.to_string())
            .header(
                "X-BAPI-RECV-WINDOW",
                s.timeout_req_ms.as_millis().to_string(),
            )
            .send()
            .await
            .map_err(|e| ExchangeError::Http(e))?;
        let time = Duration::from_millis(
            res.headers()["timenow"]
                .to_str()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
        );
        let res = res
            .json::<WRAP_REST_WITHOUT_TIME<ACC_INFO>>()
            .await
            .map_err(|e| ExchangeError::Http(e))?;
        Ok(ResultWrap {
            topic: None,
            time,
            res: res.result,
            info: None,
        })
    }
}

impl AccInfoTrait<ACC_INFO> for AccInfo {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
    ) -> impl Future<Output = Result<ResultWrap<ACC_INFO>, ExchangeError>> {
        self.retry_or_timeout.run(|| run(cl, s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exchs::bybit::prelude_tests::prelude::*;

    static ACIN: LazyLock<fn() -> AccInfo> = LazyLock::new(|| {
        || AccInfo {
            retry_or_timeout: RetryOrTimeout {
                timeout: S.timeout_cycle_ms,
            },
        }
    });

    #[tokio::test]
    async fn acc_info_req_lch_1() {
        dbg!(ACIN().run(&CL, &S).await.unwrap());
    }
}
