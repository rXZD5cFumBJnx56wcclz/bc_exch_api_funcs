#![allow(non_camel_case_types)]

use futures::{SinkExt, StreamExt};
use serde_json5::from_str;
use tokio_tungstenite::tungstenite::{self, Message};

use crate::bybit::prelude::*;

use crate::error::ExchangeError;
use crate::market::kline::*;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct DATA_KLINE {
    pub start: f64,
    pub end: f64,
    pub interval: String,
    #[serde_as(as = "DisplayFromStr")]
    pub open: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub close: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub high: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub low: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub volume: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub turnover: f64,
    pub confirm: bool,
    pub timestamp: usize,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct WRAP_KLINE {
    pub topic: String,
    pub r#type: String,
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub ts: Duration,
    pub data: Vec<DATA_KLINE>,
}

pub fn to_kline(mut msg: WRAP_KLINE) -> ResultWrap<Vec<f64>> {
    let data = msg.data.remove(0);
    ResultWrap {
        time: msg.ts,
        res: vec![
            data.start,
            data.open,
            data.open,
            data.high,
            data.low,
            data.close,
            data.volume,
            data.turnover,
        ],
        topic: Some(msg.topic.clone()),
        info: Some(data.timestamp.to_string()),
    }
}

impl Kline for BYBIT {
    fn connect_kline(
        &self,
        s: &SETTINGS_EXCH,
        symbols: &[String],
    ) -> impl Future<Output = Result<Response<Option<Vec<u8>>>, Box<dyn Error>>> {
        async move {
            let (mut ws, resp) =
                connect_wws(WS_HOST, &format!("{WS_PUBLIC}/{}", &s.category), "443").await?;
            ws.send(Message::Text(
                format!(
                    r#"{{
                        "op": "subscribe",
                        "args": {:?}
                    }}"#,
                    symbols
                        .iter()
                        .map(|v| format!("kline.{}.{}", s.timeframe.as_secs(), v,))
                        .collect::<Vec<String>>()
                )
                .into(),
            ))
            .await?;
            self.wws_connected.lock().await.insert("kline".to_string(), ws.split());
            Ok(resp)
        }
    }
    fn next_kline_req(&self) -> impl Future<Output = Option<Result<Message, tungstenite::Error>>> {
        async move {
            self.wws_connected.lock().await
                .get_mut("kline")
                .unwrap()
                .1
                .next()
                .await
        }
    }
    fn next_kline(&self) -> impl Future<Output = Result<ResultWrap<Vec<f64>>, ExchangeError>> {
        async move {
            match self.next_kline_req().await {
                Some(connected) => match connected {
                    Ok(data) => {
                        match from_str::<BybitMessagePub<WRAP_KLINE>>(&data.to_string())
                            .map_err(|e| ExchangeError::Json(e))?
                        {
                            BybitMessagePub::Data(data) => Ok(to_kline(data)),
                            BybitMessagePub::OpResponsePub(op) => match op.op.as_str() {
                                "ping" => {
                                    self.pong().await?;
                                    Err(ExchangeError::NotFindData)
                                }
                                _ => Err(ExchangeError::NotFindData),
                            },
                        }
                    }
                    Err(e) => Err(ExchangeError::WebSocket(e)),
                },
                _ => Err(ExchangeError::Closed),
            }
        }
    }

    fn next_kline_a(
        &self,
        s: &SETTINGS_EXCH,
    ) -> impl Future<Output = Result<ResultWrap<Vec<f64>>, ExchangeError>> {
        async move { all_or_nothing(async move || self.next_kline().await, s).await }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn connect_kline_res_1() {
        assert_eq_pr!(
            EXCH()
                .connect_kline(
                    &S,
                    &[
                        "SUIUSDT".to_string(),
                        "ETHUSDT".to_string(),
                        "ATOMUSDT".to_string(),
                    ],
                )
                .await
                .unwrap()
                .status()
                .as_str(),
            "101"
        );
    }

    #[tokio::test]
    async fn next_kline_res_1() {
        let exch = EXCH();
        exch.connect_kline(
            &S,
            &[
                "SUIUSDT".to_string(),
                "ETHUSDT".to_string(),
                "ATOMUSDT".to_string(),
            ],
        )
        .await
        .unwrap();
        let res = exch.next_kline().await.unwrap();
        assert!(!res.res.is_empty());
    }
}
