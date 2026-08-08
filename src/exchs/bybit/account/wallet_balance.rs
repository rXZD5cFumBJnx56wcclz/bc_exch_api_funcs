#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub use crate::account::wallet_balance::*;
use crate::exchs::bybit::prelude::*;
pub const WALLET_BALANCE: &str = "/v5/account/wallet-balance";

pub struct WalletBalance {
    pub retry_or_timeout: RetryOrTimeout,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WALLET_BALANCE1 {
    pub availableToBorrow: String,
    pub bonus: String,
    pub accruedInterest: String,
    pub availableToWithdraw: String,
    pub totalOrderIM: String,
    pub equity: String,
    pub totalPositionMM: String,
    pub usdValue: String,
    pub spotHedgingQty: String,
    pub unrealisedPnl: String,
    pub collateralSwitch: bool,
    pub borrowAmount: String,
    pub totalPositionIM: String,
    pub walletBalance: String,
    pub cumRealisedPnl: String,
    pub locked: String,
    pub marginCollateral: bool,
    pub coin: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WALLET_BALANCE {
    pub totalEquity: String,
    pub accountIMRate: String,
    pub totalMarginBalance: String,
    pub totalInitialMargin: String,
    pub accountType: String,
    pub totalAvailableBalance: String,
    pub accountMMRate: String,
    pub totalPerpUPL: String,
    pub totalWalletBalance: String,
    pub accountLTV: String,
    pub totalMaintenanceMargin: String,
    pub coin: Vec<WALLET_BALANCE1>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WRAP_WALLET_BALANCE {
    pub list: Vec<WALLET_BALANCE>,
}

fn run(
    cl: &Client,
    s: &SETTINGS_EXCH,
    coin: &str,
) -> impl Future<Output = Result<ResultWrap<Vec<WALLET_BALANCE>>, ExchangeError>> {
    async move {
        let time_stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let query = format!("accountType={}&coin={coin}", &s.account_type);
        let res = cl
            .get(format!("{}{}?{}", &s.url, WALLET_BALANCE, query))
            .header(
                "X-BAPI-SIGN",
                hmac_(
                    s.secret.as_bytes(),
                    format!(
                        "{}{}{}{}",
                        time_stamp,
                        &s.key,
                        s.timeout_req_ms.as_millis(),
                        query
                    )
                    .as_bytes(),
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
            .map_err(|e| ExchangeError::Http(e))?
            .json::<WRAP_REST<WRAP_WALLET_BALANCE>>()
            .await
            .map_err(|e| ExchangeError::Http(e))?;
        Ok(ResultWrap {
            topic: None,
            time: res.time,
            res: res.result.list,
            info: None,
        })
    }
}

impl WalletBalanceTrait<WALLET_BALANCE> for WalletBalance {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
        // optional
        coin: &str,
    ) -> impl Future<Output = Result<ResultWrap<Vec<WALLET_BALANCE>>, ExchangeError>> {
        self.retry_or_timeout.run(|| run(cl, s, coin))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exchs::bybit::prelude_tests::prelude::*;

    static WL: LazyLock<fn() -> WalletBalance> = LazyLock::new(|| {
        || WalletBalance {
            retry_or_timeout: RetryOrTimeout {
                timeout: S.timeout_cycle_ms,
            },
        }
    });

    #[tokio::test]
    async fn wallet_balance_req_lch_1() {
        dbg!(WL().run(&CL, &S, "USDT",).await.unwrap());
    }
}
