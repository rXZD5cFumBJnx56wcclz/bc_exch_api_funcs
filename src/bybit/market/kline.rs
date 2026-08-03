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

#[derive(Serialize, Deserialize, Debug)]
pub struct WRAP_KLINE {
    pub topic: String,
    pub r#type: String,
    pub ts: usize,
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
            data.turnover,
            // default index, mark
            data.close,
            data.close,
        ],
        info: Some(msg.topic.clone()),
    }
}

impl Kline for BYBIT {
    fn connect_kline(
        &mut self,
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
                        .map(|v| format!("kline.{}.{}", s.timeframe / 60000, v,))
                        .collect::<Vec<String>>()
                )
                .into(),
            ))
            .await?;
            let (sink, stream_) = ws.split();
            self.sink = Some(sink);
            self.stream_ = Some(stream_);
            Ok(resp)
        }
    }
    fn next_kline_req(
        &mut self,
    ) -> impl Future<Output = Option<Result<Message, tungstenite::Error>>> {
        async move { self.stream_.as_mut().unwrap().next().await }
    }
    fn next_kline(&mut self) -> impl Future<Output = Result<ResultWrap<Vec<f64>>, ExchangeError>> {
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
                                    self.pong().await.map_err(|e| ExchangeError::WebSocket(e))?;
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
        &mut self,
        s: &SETTINGS_EXCH,
    ) -> impl Future<Output = Result<ResultWrap<Vec<f64>>, ExchangeError>> {
        async move {
            let instant = Instant::now();
            loop {
                if instant.duration_since(Instant::now()).as_millis() as usize > s.timeout_cycle_ms
                {
                    return Err(ExchangeError::Timeout);
                }
                if let Ok(res) = self.next_kline().await {
                    return Ok(res);
                } else {
                    dbg!("err");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn connect_kline_res_1() {
        assert_eq_pr!(EXCH()
            .connect_kline(
                &S,
                &[
                    "SUIUSDT".to_string(),
                    "ETHUSDT".to_string(),
                    "ATOMUSDT".to_string(),
                ],
            )
            .await
            .unwrap().status().as_str(), "101");
    }

    #[tokio::test]
    async fn next_kline_res_1() {
        let mut exch = EXCH();
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
