use bc_exch_api_funcs::bybit::market::symbols::*;

use crate::unit::bybit::prelude::*;

#[tokio::test]
async fn symbols_req_lch_1() {
    EXCH.symbols_req("", "", "").await.unwrap();
}

#[tokio::test]
async fn symbols_a_lch_1() {
    EXCH.symbols_a("", "", "").await.unwrap();
}
