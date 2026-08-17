#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::exchs::bybit::prelude::*;
use crate::market::tickers::TickersTrait;
pub const TICKERS: &str = "/v5/market/tickers";

pub struct Tickers {
    pub retry_or_timeout: RetryOrTimeout,
}

impl Tickers {
    pub fn new(s: &SETTINGS_EXCH) -> Self {
        Self {
            retry_or_timeout: RetryOrTimeout {
                timeout: s.timeout_cycle_ms,
            },
        }
    }
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct TICKERS1 {
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
pub struct WRAP_TICKERS {
    pub category: String,
    pub list: Vec<TICKERS1>,
}

fn run(
    cl: &Client,
    s: &SETTINGS_EXCH,
    base_coin: &str,
    exp_date: &str,
) -> impl Future<Output = Result<Wrap<Vec<TICKERS1>>, ExchangeError>> {
    async move {
        let req = cl
            .get(format!(
                "{}{TICKERS}\
                        ?category={}\
                        &baseCoin={base_coin}\
                        &expDate={exp_date}",
                &s.url, &s.category,
            ))
            .send()
            .await
            .map_err(|e| ExchangeError::Http(e))?
            .json::<WRAP_REST<WRAP_TICKERS>>()
            .await
            .map_err(|e| ExchangeError::Http(e))?;
        Ok(Wrap {
            time: req.time,
            res: req.result.list,
            info: None,
            topic: None,
        })
    }
}

impl TickersTrait<TICKERS1> for Tickers {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<Wrap<Vec<TICKERS1>>, ExchangeError>> {
        async move {
            self.retry_or_timeout
                .run(|| run(cl, s, base_coin, exp_date))
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::exchs::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn tickers_res_1() {
        assert!(
            !Tickers::new(&S)
                .run(&CL, &S, "", "")
                .await
                .unwrap()
                .res
                .is_empty()
        )
    }
}
