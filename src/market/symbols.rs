#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::prelude::*;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct SYMBOLS1 {
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
pub struct RESULT_WRAP_SYMBOLS {
    pub category: String,
    pub list: Vec<SYMBOLS1>,
}

impl ResultWrap<Vec<SYMBOLS1>> for RESULT_WRAP_SYMBOLS {
    fn res(self) -> Vec<SYMBOLS1> {
        self.list
    }
}

pub trait Symbols: Exchange {
    fn symbols_req<'a, T>(
        &'a self,
        symbol: &str,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<impl ResultWrap<T>, Error_req>>
    where
        T: ResultWrap<Vec<SYMBOLS1>>;
    fn symbols<'a>(
        &'a self,
        symbol: &str,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<Vec<SYMBOLS1>, Box<dyn std::error::Error>>> {
        async move {
            Ok(self
                .symbols_req::<RESULT_WRAP_SYMBOLS>(symbol, base_coin, exp_date)
                .await?
                .res()
                .res())
        }
    }
    fn symbols_a<'a>(
        &'a self,
        symbol: &str,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<Vec<SYMBOLS1>, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                async || self.symbols(symbol, base_coin, exp_date).await,
                usizezero(self.s().exch.timeout_cycle_ms),
            )
            .await
        }
    }
}
