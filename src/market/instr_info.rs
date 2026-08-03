#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use bc_utils_lg::structs::settings::SETTINGS_EXCH;

use crate::prelude::*;

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

ResultWrap<Vec<INSTR_INFO1>> for INSTR_INFO {
    fn res(self) -> Vec<INSTR_INFO1> {
        self.list
    }
}

impl GetTime for INSTR_INFO {
    fn time(&self) -> usize {
        0
    }
}

pub trait InstrumentsInfo {
    fn instr_info_req(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        status: &str,
        base_coin: &str,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<ResultWrap<INSTR_INFO>, Error_req>>;
    fn instr_info(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        status: &str,
        base_coin: &str,
    ) -> impl Future<Output = Result<INSTR_INFO1, Box<dyn std::error::Error>>> {
        async move {
            self.instr_info_req(s, symbol, status, base_coin, 1, "")
                .await?
                .res()
                .res()
                .into_iter()
                .next()
                .ok_or(Box::from("not found"))
        }
    }

    fn instr_info_a(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        status: &str,
        base_coin: &str,
    ) -> impl Future<Output = Result<INSTR_INFO1, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                async || self.instr_info(s, symbol, status, base_coin).await,
                usizezero(s.timeout_cycle_ms),
            )
            .await
        }
    }

    fn instrs_info<'a>(
        &self,
        s: &SETTINGS_EXCH,
        symbols: &'a [String],
        status: &'a str,
        base_coin: &'a str,
    ) -> impl Future<Output = Result<MAP<&'a str, INSTR_INFO1>, Box<dyn std::error::Error>>> {
        async move {
            let mut res = MAP::default();
            let mut passed = vec![];
            let mut cursor = "".to_string();
            while passed.len() != symbols.len() {
                let response_ = self
                    .instr_info_req(
                        s, "", status, base_coin, // fix this `limit` arg ↓
                        1000, &cursor,
                    )
                    .await?
                    .res();
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

    fn instrs_info_a<'a>(
        &self,
        s: &SETTINGS_EXCH,
        symbols: &'a [String],
        status: &'a str,
        base_coin: &'a str,
    ) -> impl Future<Output = Result<MAP<&'a str, INSTR_INFO1>, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                || self.instrs_info(s, symbols, status, base_coin),
                usizezero(s.timeout_cycle_ms),
            )
            .await
        }
    }
}
