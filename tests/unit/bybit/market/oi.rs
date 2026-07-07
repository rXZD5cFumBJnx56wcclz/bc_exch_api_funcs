use crate::unit::bybit::prelude::*;
use bc_exch_api_funcs::bybit::market::oi::*;

#[tokio::test]
async fn oi_req_lch_1() {
    println!("{:#?}", EXCH.oi_req("SUIUSDT", "5min", 0, 0, 1, "",).await);
}

#[tokio::test]
async fn oi_a_lch_1() {
    println!(
        "{:#?}",
        EXCH.oi_a("SUIUSDT", "5min", 0, 0, 1, "",).await.unwrap()
    );
}
