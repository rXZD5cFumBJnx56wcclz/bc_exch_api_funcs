use std::error::Error;
use std::time::Duration;

use reqwest::{
    Client, 
    Error as Error_req,
};
use bc_utils_lg::structs::exch::bybit::instr_info::{
    RESULT_INSTR_INFO,
    RESULT_INSTR_INFO1, 
};
use bc_utils_lg::structs::exch::bybit::result::RESULT_EXCH_BYBIT;
use bc_utils_core::mechanisms::all_or_nothing;
use bc_utils_lg::types::maps::MAP;

use crate::bybit::const_url::INSTR_INFO;
use crate::deffunc::usizezero;


pub async fn instr_info_req(
    api_url: &str,
    category: &str,
    symbol: &str,
    status: &str,
    base_coin: &str,
    limit: &usize,
    cursor: &str,
    timeout_ms: &Duration,
) -> Result<RESULT_EXCH_BYBIT<RESULT_INSTR_INFO>, Error_req>
{
    Client::builder()
        .timeout(*timeout_ms)
        .build()
        ?
        .get(format!(
            "{api_url}{INSTR_INFO}\
            ?category={category}\
            &symbol={symbol}\
            &status={status}\
            &baseCoin={base_coin}\
            &limit={limit}\
            &cursor={cursor}"
        ))
        .send()
        .await?
        .json::<RESULT_EXCH_BYBIT<RESULT_INSTR_INFO>>()
        .await
}

pub async fn instr_info(
    api_url: &str,
    category: &str,
    symbol: &str,
    status: &str,
    base_coin: &str,
    timeout_ms: &usize,
) -> Result<RESULT_INSTR_INFO1, Box<dyn std::error::Error>>
{
    instr_info_req(
        api_url, 
        category, 
        symbol, 
        status, 
        base_coin, 
        &1,
        "",
        &Duration::from_millis(*usizezero(timeout_ms) as u64),
    ).await?.result.list.into_iter().next().ok_or(Box::from("not found"))
}

pub async fn instr_info_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    status: &str,
    base_coin: &str,
    timeout_ms: &usize,
    timeout_cycle_ms: &usize,
) -> Result<RESULT_INSTR_INFO1, Box<dyn Error>>
{
    all_or_nothing(
        async || instr_info(
            api_url, 
            category, 
            symbol, 
            status, 
            base_coin, 
            timeout_ms,
        ).await,
        timeout_cycle_ms,
    ).await
}

pub async fn instrs_info<'a>(
    api_url: &'a str,
    category: &'a str,
    symbols: &'a [String],
    status: &'a str,
    base_coin: &'a str,
    timeout_ms: &usize,
) -> Result<MAP<&'a str, RESULT_INSTR_INFO1>, Box<dyn std::error::Error>> 
{
    let timeout_ms = Duration::from_millis(*usizezero(timeout_ms) as u64);
    let mut res = MAP::default();
    let mut passed = vec![];
    let mut cursor = "".to_string();
    while passed.len() != symbols.len() {
        let response_ = instr_info_req(
            api_url, 
            category, 
            "", 
            status, 
            base_coin, 
            // fix this `limit` arg ↓
            &1000,
            &cursor,
            &timeout_ms,
        )
            .await?.result;
        cursor = response_.nextPageCursor.clone();
        for v in  response_.list.into_iter(){
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

pub async fn instrs_info_a<'a>(
    api_url: &'a str,
    category: &'a str,
    symbols: &'a [String],
    status: &'a str,
    base_coin: &'a str,
    timeout_ms: &usize,
    timeout_cycle_ms: &usize,
) -> Result<MAP<&'a str, RESULT_INSTR_INFO1>, Box<dyn Error>>
{
    all_or_nothing(
        || instrs_info(
            api_url, 
            category, 
            symbols, 
            status, 
            base_coin,
            timeout_ms,
        ),
        timeout_cycle_ms,
    ).await
}
