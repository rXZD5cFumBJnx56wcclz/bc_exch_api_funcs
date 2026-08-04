#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_with::{DurationMilliSeconds, serde_as};

#[derive(Serialize, Deserialize, Debug)]
#[serde_as]
pub struct RESULT_EXCH_BYBIT<T> {
    pub retCode: i32,
    pub retMsg: String,
    pub result: T,
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub time: Duration,
}

#[derive(Deserialize, Debug)]
pub struct OpResponsePub {
    pub success: bool,
    pub ret_msg: String,
    pub conn_id: String,
    pub req_id: String,
    pub op: String,
}

#[derive(Deserialize, Debug)]
pub struct OpResponsePrivate {
    pub args: Vec<String>,
    pub ret_msg: String,
    pub conn_id: String,
    pub req_id: String,
    pub op: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum BybitMessagePub<T> {
    Data(T),
    OpResponsePub(OpResponsePub),
}
