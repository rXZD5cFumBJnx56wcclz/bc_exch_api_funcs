use bc_exch_api_funcs::bybit::account::acc_info::*;

use crate::unit::bybit::prelude::*;

#[tokio::test]
async fn acc_info_req_lch_1() {
    println!("{:#?}", EXCH.acc_info_req().await.unwrap());
}

#[tokio::test]
async fn acc_info_a_lch_1() {
    println!("{:#?}", EXCH.acc_info_a().await.unwrap());
}
