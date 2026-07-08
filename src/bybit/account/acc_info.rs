#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub use crate::account::acc_info::*;
use crate::bybit::prelude::*;
pub const ACC_INFO: &str = "/v5/account/info";

impl ResultWrap<ACC_INFO> for RESULT_EXCH_BYBIT<ACC_INFO> {
    fn res(self) -> ACC_INFO {
        self.result
    }
}

impl AccInfo for BYBIT<'_> {
    fn acc_info_req<'a>(
        &'a self
    ) -> impl Future<Output = Result<impl ResultWrap<ACC_INFO>, Error_req>> {
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
                        format!(
                            "{}{}{}",
                            time_stamp, &self.s.exch.key, self.s.exch.timeout_req_ms
                        )
                        .as_bytes(),
                    ),
                )
                .header("X-BAPI-API-KEY", &self.s.exch.key)
                .header("X-BAPI-TIMESTAMP", time_stamp.to_string())
                .header("X-BAPI-RECV-WINDOW", self.s.exch.timeout_req_ms)
                .send()
                .await?
                .json::<RESULT_EXCH_BYBIT<ACC_INFO>>()
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bybit::account::acc_info::*;
    use crate::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn acc_info_req_lch_1() {
        println!("{:#?}", EXCH.acc_info_req().await.unwrap());
    }

    #[tokio::test]
    async fn acc_info_a_lch_1() {
        println!("{:#?}", EXCH.acc_info_a().await.unwrap());
    }
}
