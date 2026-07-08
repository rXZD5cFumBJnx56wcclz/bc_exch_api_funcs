#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub use crate::account::wallet_balance::*;
use crate::bybit::prelude::*;
pub const WALLET_BALANCE: &str = "/v5/account/wallet-balance";

#[derive(Deserialize, Serialize, Debug)]
pub struct WRAP_WALLET_BALANCE {
    pub list: Vec<WALLET_BALANCE>,
}

impl ResultWrap<Vec<WALLET_BALANCE>> for RESULT_EXCH_BYBIT<WRAP_WALLET_BALANCE> {
    fn res(self) -> Vec<WALLET_BALANCE> {
        self.result.list
    }
}

impl WalletBalance for BYBIT<'_> {
    fn wallet_balance_req<'a>(
        &'a self,

        coin: &str,
    ) -> impl Future<Output = Result<impl ResultWrap<Vec<WALLET_BALANCE>>, Error_req>> {
        async move {
            let time_stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let query = format!("accountType={}&coin={coin}", &self.s.trade.account_type);
            self.client
                .get(format!("{}{}?{}", &self.s.exch.url, WALLET_BALANCE, query))
                .header(
                    "X-BAPI-SIGN",
                    hmac_(
                        self.s.exch.secret.as_bytes(),
                        format!(
                            "{}{}{}{}",
                            time_stamp, &self.s.exch.key, self.s.exch.timeout_req_ms, query
                        )
                        .as_bytes(),
                    ),
                )
                .header("X-BAPI-API-KEY", &self.s.exch.key)
                .header("X-BAPI-TIMESTAMP", time_stamp.to_string())
                .header("X-BAPI-RECV-WINDOW", self.s.exch.timeout_req_ms)
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
        println!("{:#?}", EXCH.wallet_balance_req("USDT",).await.unwrap());
    }

    #[tokio::test]
    async fn wallet_balance_a_lch_1() {
        println!("{:#?}", EXCH.wallet_balance_a("USDT", 0,).await.unwrap());
    }
}
