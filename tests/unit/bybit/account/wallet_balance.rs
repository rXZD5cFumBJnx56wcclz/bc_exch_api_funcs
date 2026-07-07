use crate::unit::bybit::prelude::*;
use bc_exch_api_funcs::bybit::account::wallet_balance::*;

#[tokio::test]
async fn wallet_balance_req_lch_1() {
    println!("{:#?}", EXCH.wallet_balance_req("USDT",).await.unwrap());
}

#[tokio::test]
async fn wallet_balance_a_lch_1() {
    println!("{:#?}", EXCH.wallet_balance_a("USDT", 0,).await.unwrap());
}
