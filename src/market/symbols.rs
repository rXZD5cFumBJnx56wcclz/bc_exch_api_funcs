#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::const_url::TICKERS;
use crate::bybit::prelude::*;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_SYMBOLS1 {
    pub symbol: String,
    pub lastPrice: String,
    pub indexPrice: String,
    pub markPrice: String,
    pub prevPrice24h: String,
    pub price24hPcnt: String,
    pub highPrice24h: String,
    pub lowPrice24h: String,
    pub prevPrice1h: String,
    pub openInterest: String,
    pub openInterestValue: String,
    pub turnover24h: String,
    pub volume24h: String,
    pub fundingRate: String,
    pub nextFundingTime: String,
    pub predictedDeliveryPrice: String,
    pub basisRate: String,
    pub deliveryFeeRate: String,
    pub deliveryTime: String,
    pub ask1Size: String,
    pub bid1Price: String,
    pub ask1Price: String,
    pub bid1Size: String,
    pub basis: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_SYMBOLS {
    pub category: String,
    pub list: Vec<RESULT_SYMBOLS1>,
}

pub trait Symbols: Exchange {
    fn symbols_req<'a>(
        &'a self,
        symbol: &str,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<impl ResultWrap<RESULT_SYMBOLS>, Error_req>>;
    fn symbols<'a>(
        &'a self,
        symbol: &str,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<Vec<RESULT_SYMBOLS1>, Box<dyn std::error::Error>>> {
        async move {
            Ok(self
                .symbols_req(symbol, base_coin, exp_date)
                .await?
                .result
                .list)
        }
    }
    fn symbols_a<'a>(
        &'a self,
        symbol: &str,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<Vec<RESULT_SYMBOLS1>, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                async || self.symbols(symbol, base_coin, exp_date).await,
                usizezero(self.s().exch.timeout_cycle_ms),
            )
            .await
        }
    }
}
