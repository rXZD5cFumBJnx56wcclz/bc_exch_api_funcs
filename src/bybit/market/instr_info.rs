#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::const_url::INSTR_INFO;
use crate::bybit::prelude::*;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_INSTR_INFO2_LEVERAGE_FILTER {
    pub minLeverage: String,
    pub maxLeverage: String,
    pub leverageStep: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_INSTR_INFO2_PRICE_FILTER {
    pub minPrice: String,
    pub maxPrice: String,
    pub tickSize: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_INSTR_INFO2_LOT_SIZE_FILTER {
    pub maxOrderQty: String,
    pub minOrderQty: String,
    pub qtyStep: String,
    pub postOnlyMaxOrderQty: String,
    pub maxMktOrderQty: String,
    pub minNotionalValue: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_INSTR_INFO2_RISK_PARAMETERS {
    pub priceLimitRatioX: String,
    pub priceLimitRatioY: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_INSTR_INFO1 {
    pub symbol: String,
    pub contractType: String,
    pub status: String,
    pub baseCoin: String,
    pub quoteCoin: String,
    pub launchTime: String,
    pub deliveryTime: String,
    pub deliveryFeeRate: String,
    pub priceScale: String,
    pub leverageFilter: RESULT_INSTR_INFO2_LEVERAGE_FILTER,
    pub priceFilter: RESULT_INSTR_INFO2_PRICE_FILTER,
    pub lotSizeFilter: RESULT_INSTR_INFO2_LOT_SIZE_FILTER,
    pub unifiedMarginTrade: bool,
    pub fundingInterval: i32,
    pub settleCoin: String,
    pub copyTrading: String,
    pub upperFundingRate: String,
    pub lowerFundingRate: String,
    pub isPreListing: bool,
    pub preListingInfo: Option<String>,
    pub riskParameters: RESULT_INSTR_INFO2_RISK_PARAMETERS,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_INSTR_INFO {
    pub category: String,
    pub list: Vec<RESULT_INSTR_INFO1>,
    pub nextPageCursor: String,
}

pub trait InstrumentsInfo<'a>: Exchange<'a> {
    fn instr_info_req(
        &'a self,
        symbol: &str,
        status: &str,
        base_coin: &str,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<RESULT_EXCH_BYBIT<RESULT_INSTR_INFO>, Error_req>>;
    fn instr_info(
        &'a self,
        symbol: &str,
        status: &str,
        base_coin: &str,
    ) -> impl Future<Output = Result<RESULT_INSTR_INFO1, Box<dyn std::error::Error>>> {
        async move {
            self.instr_info_req(symbol, status, base_coin, 1, "")
                .await?
                .result
                .list
                .into_iter()
                .next()
                .ok_or(Box::from("not found"))
        }
    }

    fn instr_info_a(
        &'a self,
        symbol: &str,
        status: &str,
        base_coin: &str,
    ) -> impl Future<Output = Result<RESULT_INSTR_INFO1, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                async || self.instr_info(symbol, status, base_coin).await,
                usizezero(self.s().exch.timeout_cycle_ms),
            )
            .await
        }
    }

    fn instrs_info(
        &'a self,
        symbols: &'a [String],
        status: &'a str,
        base_coin: &'a str,
    ) -> impl Future<Output = Result<MAP<&'a str, RESULT_INSTR_INFO1>, Box<dyn std::error::Error>>>
    {
        async move {
            let mut res = MAP::default();
            let mut passed = vec![];
            let mut cursor = "".to_string();
            while passed.len() != symbols.len() {
                let response_ = self
                    .instr_info_req(
                        "", status, base_coin, // fix this `limit` arg ↓
                        1000, &cursor,
                    )
                    .await?
                    .result;
                cursor = response_.nextPageCursor.clone();
                for v in response_.list.into_iter() {
                    for s in symbols {
                        if s == &v.symbol {
                            res.insert(s.as_str(), v);
                            passed.push(s.as_str());
                            break;
                        }
                    }
                }
            }
            Ok(res)
        }
    }

    fn instrs_info_a(
        &'a self,
        symbols: &'a [String],
        status: &'a str,
        base_coin: &'a str,
    ) -> impl Future<Output = Result<MAP<&'a str, RESULT_INSTR_INFO1>, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                || self.instrs_info(symbols, status, base_coin),
                usizezero(self.s().exch.timeout_cycle_ms),
            )
            .await
        }
    }
}

impl<'a> InstrumentsInfo<'a> for BYBIT<'a> {
    fn instr_info_req(
        &'a self,
        symbol: &str,
        status: &str,
        base_coin: &str,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<RESULT_EXCH_BYBIT<RESULT_INSTR_INFO>, Error_req>> {
        async move {
            self.client
                .get(format!(
                    "{}{INSTR_INFO}\
                ?category={}\
                &symbol={symbol}\
                &status={status}\
                &baseCoin={base_coin}\
                &limit={limit}\
                &cursor={cursor}",
                    &self.s().exch.url,
                    &self.s().trade.category,
                ))
                .send()
                .await?
                .json::<RESULT_EXCH_BYBIT<RESULT_INSTR_INFO>>()
                .await
        }
    }
}
