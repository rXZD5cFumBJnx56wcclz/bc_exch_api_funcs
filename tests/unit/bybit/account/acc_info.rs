use bc_utils_lg::funcs::settings::settings_from_json;
use reqwest::Client;

use bc_exch_api_funcs::bybit::account::acc_info::*;

#[tokio::test]
async fn acc_info_req_lch_1() {
    let sttngs = settings_from_json("settings.json").unwrap();
    println!(
        "{:#?}",
        acc_info_req(
            &Client::new(),
            &sttngs.exch.key,
            &sttngs.exch.secret,
            &sttngs.exch.url,
        )
        .await
        .unwrap()
    );
}

#[tokio::test]
async fn acc_info_a_lch_1() {
    let sttngs = settings_from_json("settings.json").unwrap();
    println!(
        "{:#?}",
        acc_info_a(
            &Client::new(),
            &sttngs.exch.key,
            &sttngs.exch.secret,
            &sttngs.exch.url,
            0,
        )
        .await
        .unwrap()
    );
}
