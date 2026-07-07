#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::bybit::prelude::*;

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

pub trait WalletBalance: Exchange {
    fn wallet_balance_req<'a>(
        &'a self,

        coin: &str,
    ) -> impl Future<Output = Result<impl ResultWrap<Vec<WALLET_BALANCE>>, Error_req>>;
    fn wallet_balance<'a>(
        &'a self,

        coin: &str,
    ) -> impl Future<Output = Result<Vec<WALLET_BALANCE>, Box<dyn std::error::Error>>> {
        async move { Ok(self.wallet_balance_req(coin).await?.res()) }
    }

    fn wallet_balance_a<'a>(
        &'a self,

        coin: &str,
        timeout_cycle_ms: usize,
    ) -> impl Future<Output = Result<Vec<WALLET_BALANCE>, Box<dyn Error>>> {
        async move { all_or_nothing(|| self.wallet_balance(coin), timeout_cycle_ms).await }
    }
}
