#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use bc_utils_core::hashing::hmac_;
use bc_utils_core::mechanisms::all_or_nothing;
use reqwest::{Client, Error as Error_req};

use crate::bybit::const_url::WALLET_BALANCE;
use crate::bybit::exch_struct::{BYBIT, Exchange};
use crate::bybit::result_req::RESULT_EXCH_BYBIT;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RESULT_WALLET_BALANCE2 {
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
pub struct RESULT_WALLET_BALANCE1 {
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
    pub coin: Vec<RESULT_WALLET_BALANCE2>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RESULT_WALLET_BALANCE {
    pub list: Vec<RESULT_WALLET_BALANCE1>,
}

pub trait WalletBalance<'a>: Exchange<'a> {
    fn wallet_balance_req(
        &'a self,
        client: &Client,
        // fix
        account_type: &str,
        coin: &str,
    ) -> impl Future<Output = Result<RESULT_EXCH_BYBIT<RESULT_WALLET_BALANCE>, Error_req>>;
    fn wallet_balance(
        &'a self,
        client: &Client,
        account_type: &str,
        coin: &str,
    ) -> impl Future<Output = Result<Vec<RESULT_WALLET_BALANCE1>, Box<dyn std::error::Error>>> {
        async move {
            Ok(self
                .wallet_balance_req(client, account_type, coin)
                .await?
                .result
                .list)
        }
    }

    fn wallet_balance_a(
        &'a self,
        client: &Client,
        account_type: &str,
        coin: &str,
        timeout_cycle_ms: usize,
    ) -> impl Future<Output = Result<Vec<RESULT_WALLET_BALANCE1>, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                || self.wallet_balance(client, account_type, coin),
                timeout_cycle_ms,
            )
            .await
        }
    }
}

impl<'a> WalletBalance<'a> for BYBIT<'a> {
    fn wallet_balance_req(
        &'a self,
        client: &Client,
        account_type: &str,
        coin: &str,
    ) -> impl Future<Output = Result<RESULT_EXCH_BYBIT<RESULT_WALLET_BALANCE>, Error_req>> {
        async move {
            let time_stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let query = format!("accountType={account_type}&coin={coin}");
            client
                .get(format!("{}{}?{}", &self.s.exch.url, WALLET_BALANCE, query))
                .header(
                    "X-BAPI-SIGN",
                    hmac_(
                        self.s.exch.secret.as_bytes(),
                        format!("{}{}{}{}", time_stamp, &self.s.exch.key, 5000, query).as_bytes(),
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
