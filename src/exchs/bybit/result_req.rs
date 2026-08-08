#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::{DurationMilliSeconds, serde_as};

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct WRAP_REST<T> {
    pub retCode: i32,
    pub retMsg: String,
    pub result: T,
    #[serde_as(as = "DurationMilliSeconds<u64, Flexible>")]
    pub time: Duration,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct WRAP_WWS<T> {
    pub topic: String,
    pub r#type: String,
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub ts: Duration,
    pub data: Vec<T>,
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
pub enum MsgPub<T> {
    Data(T),
    OpResponsePub(OpResponsePub),
}
