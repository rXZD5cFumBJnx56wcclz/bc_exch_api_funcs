#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::prelude::*;

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

pub trait Symbols {
    fn symbols(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<ResultWrap<Vec<SYMBOLS1>>, ExchangeError>>;
    fn symbols_a(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<ResultWrap<Vec<SYMBOLS1>>, ExchangeError>> {
        async move {
            all_or_nothing(
                async || Ok(self.symbols(s, symbol, base_coin, exp_date).await?),
                s,
            )
            .await
        }
    }
    fn symbols_only(
        &self,
        s: &SETTINGS_EXCH,
    ) -> impl Future<Output = Result<ResultWrap<Vec<String>>, ExchangeError>> {
        async move {
            let res = self.symbols(s, "", "", "").await?;
            Ok(ResultWrap {
                time: res.time,
                res: res.res.into_iter().map(|v| v.symbol).collect(),
                info: None,
                topic: None,
            })
        }
    }
}
