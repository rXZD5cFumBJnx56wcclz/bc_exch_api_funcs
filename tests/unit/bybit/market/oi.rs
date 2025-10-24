use std::time::Duration;

use tokio;

use bc_exch_api_funcs::bybit::market::oi::*;


#[tokio::test]
async fn oi_req_lch_1(){
    println!("{:#?}", oi_req(
        "https://api.bybit.com", 
        "linear",
        "SUIUSDT", 
        "5min", 
        &0, 
        &0, 
        &1, 
        "",
        &Duration::from_millis(5000u64),
    ).await);
}

#[tokio::test]
async fn oi_a_lch_1(){
    println!("{:#?}", oi_a(
        "https://api.bybit.com", 
        "linear",
        "SUIUSDT", 
        "5min", 
        &0, 
        &0, 
        &1, 
        "",
        &0,
        &0,
    ).await.unwrap());
}
