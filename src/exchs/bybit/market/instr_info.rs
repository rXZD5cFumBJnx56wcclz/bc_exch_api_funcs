#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::exchs::bybit::prelude::*;
pub use crate::market::instr_info::*;

pub const INSTR_INFO: &str = "/v5/market/instruments-info";

pub struct InstrInfo {
    pub retry_or_timeout: RetryOrTimeout,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct INSTR_INFO2_LEVERAGE_FILTER {
    pub minLeverage: String,
    pub maxLeverage: String,
    pub leverageStep: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct INSTR_INFO2_PRICE_FILTER {
    pub minPrice: String,
    pub maxPrice: String,
    pub tickSize: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct INSTR_INFO2_LOT_SIZE_FILTER {
    pub maxOrderQty: String,
    pub minOrderQty: String,
    pub qtyStep: String,
    pub postOnlyMaxOrderQty: String,
    pub maxMktOrderQty: String,
    pub minNotionalValue: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct INSTR_INFO2_RISK_PARAMETERS {
    pub priceLimitRatioX: String,
    pub priceLimitRatioY: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct INSTR_INFO1 {
    pub symbol: String,
    pub contractType: String,
    pub status: String,
    pub baseCoin: String,
    pub quoteCoin: String,
    pub launchTime: String,
    pub deliveryTime: String,
    pub deliveryFeeRate: String,
    pub priceScale: String,
    pub leverageFilter: INSTR_INFO2_LEVERAGE_FILTER,
    pub priceFilter: INSTR_INFO2_PRICE_FILTER,
    pub lotSizeFilter: INSTR_INFO2_LOT_SIZE_FILTER,
    pub unifiedMarginTrade: bool,
    pub fundingInterval: i32,
    pub settleCoin: String,
    pub copyTrading: String,
    pub upperFundingRate: String,
    pub lowerFundingRate: String,
    pub isPreListing: bool,
    pub preListingInfo: Option<String>,
    pub riskParameters: INSTR_INFO2_RISK_PARAMETERS,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct INSTR_INFO {
    pub category: String,
    pub list: Vec<INSTR_INFO1>,
    pub nextPageCursor: String,
}

async fn run(
    cl: &Client,
    s: &SETTINGS_EXCH,
    symbol: &str,
    base_coin: &str,
    limit: usize,
    cursor: &str,
) -> Result<ResultWrap<MAP<String, INSTR_INFO1>>, ExchangeError> {
    let res = cl
        .get(format!(
            "{}{INSTR_INFO}\
                    ?category={}\
                    &symbol={symbol}\
                    &baseCoin={base_coin}\
                    &limit={limit}\
                    &cursor={cursor}",
            &s.url, &s.category,
        ))
        .send()
        .await
        .map_err(|e| ExchangeError::Http(e))?
        .json::<WRAP_REST<INSTR_INFO>>()
        .await
        .map_err(|e| ExchangeError::Http(e))?;
    Ok(ResultWrap {
        topic: None,
        time: res.time,
        res: res
            .result
            .list
            .into_iter()
            .map(|v| (v.symbol.clone(), v))
            .collect(),
        info: Some(res.result.nextPageCursor),
    })
}

async fn run_more_1000(
    cl: &Client,
    s: &SETTINGS_EXCH,
    base_coin: &str,
    limit: usize,
    retry_or_timeout: &RetryOrTimeout,
) -> Result<ResultWrap<MAP<String, INSTR_INFO1>>, ExchangeError> {
    let mut limit_left = limit;
    let mut cursor = String::new();
    let mut res = MAP::default();
    let mut time_last = Default::default();
    for _ in 0..usize::MAX {
        let limit_resp = limit_left / 1000;
        let resp = if limit_resp != 0 {
            retry_or_timeout
                .run(async || run(cl, s, "", base_coin, limit_resp, &cursor).await)
                .await?
        } else {
            retry_or_timeout
                .run(async || run(cl, s, "", base_coin, limit_left, &cursor).await)
                .await?
        };
        cursor = resp.info.unwrap().clone();
        time_last = resp.time;
        res.extend(resp.res);
        limit_left -= limit_resp;
    }
    Ok(ResultWrap {
        topic: None,
        time: time_last,
        res,
        info: None,
    })
}

impl InstrumentsInfoTrait<INSTR_INFO1> for InstrInfo {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
        symbol: &str,
        base_coin: &str,
        limit: usize,
    ) -> impl Future<Output = Result<ResultWrap<MAP<String, INSTR_INFO1>>, ExchangeError>> {
        async move {
            if !symbol.is_empty() {
                return self
                    .retry_or_timeout
                    .run(async || run(cl, s, symbol, base_coin, 1, "").await)
                    .await;
            }
            if limit > 1000 {
                return run_more_1000(cl, s, base_coin, limit, &self.retry_or_timeout).await;
            }
            return self
                .retry_or_timeout
                .run(async || run(cl, s, "", base_coin, limit, "").await)
                .await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exchs::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn instr_info_res_1() {
        assert_eq!(
            InstrInfo {
                retry_or_timeout: RetryOrTimeout {
                    timeout: S.timeout_cycle_ms
                }
            }
            .run(&CL, &S, "", "", 100)
            .await
            .unwrap()
            .res
            .len(),
            100
        )
    }
}
