#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub use crate::account::wallet_balance::*;
use crate::bybit::prelude::*;
pub const WALLET_BALANCE: &str = "/v5/account/wallet-balance";

#[derive(Deserialize, Serialize, Debug)]
pub struct WRAP_WALLET_BALANCE {
    pub list: Vec<WALLET_BALANCE>,
}

ResultWrap<Vec<WALLET_BALANCE>> for RESULT_EXCH_BYBIT<WRAP_WALLET_BALANCE> {
    fn res(self) -> Vec<WALLET_BALANCE> {
        self.result.list
    }
}

impl WalletBalance for BYBIT {
    fn wallet_balance_req(
        &self,
        s: &SETTINGS_EXCH,
        coin: &str,
    ) -> impl Future<Output = Result<ResultWrap<Vec<WALLET_BALANCE>>, Error_req>> {
        async move {
            let time_stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let query = format!("accountType={}&coin={coin}", &s.account_type);
            self.client
                .get(format!("{}{}?{}", &s.url, WALLET_BALANCE, query))
                .header(
                    "X-BAPI-SIGN",
                    hmac_(
                        s.secret.as_bytes(),
                        format!("{}{}{}{}", time_stamp, &s.key, s.timeout_req_ms, query).as_bytes(),
                    ),
                )
                .header("X-BAPI-API-KEY", &s.key)
                .header("X-BAPI-TIMESTAMP", time_stamp.to_string())
                .header("X-BAPI-RECV-WINDOW", s.timeout_req_ms)
                .send()
                .await?
                .json::<RESULT_EXCH_BYBIT<WRAP_WALLET_BALANCE>>()
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bybit::account::wallet_balance::*;
    use crate::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn wallet_balance_req_lch_1() {
        println!(
            "{:#?}",
            EXCH().wallet_balance_req(&S, "USDT",).await.unwrap()
        );
    }

    #[tokio::test]
    async fn wallet_balance_a_lch_1() {
        println!(
            "{:#?}",
            EXCH().wallet_balance_a(&S, "USDT", 0,).await.unwrap()
        );
    }
}
