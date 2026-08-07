#![allow(non_camel_case_types)]

use crate::{bybit::prelude::*, market::kline::Kline as KlineTrait};

pub struct Kline {
    pub conn: BybitConnection,
    pub retry_or_timeout: RetryOrTimeout,
}

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

pub fn to_kline_and_check(
    mut msg: WRAP_WWS<DATA_KLINE>,
) -> Result<ResultWrap<Vec<f64>>, ExchangeError> {
    let data = msg.data.remove(0);
    if data.confirm {
        return Err(ExchangeError::NotFindData);
    }
    Ok(ResultWrap {
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
    })
}

fn next(conn: &Connection) -> impl Future<Output = Result<ResultWrap<Vec<f64>>, ExchangeError>> {
    Box::pin(async move {
        match conn
            .0
            .lock()
            .await
            .1
            .next()
            .await
            .ok_or(ExchangeError::Disconnected)?
            .map_err(|e| ExchangeError::WebSocket(e))?
        {
            Message::Text(text) => match from_str::<MsgPub<WRAP_WWS<DATA_KLINE>>>(&text)
                .map_err(|e| ExchangeError::Json(e))?
            {
                MsgPub::Data(v) => to_kline_and_check(v),
                _ => Err(ExchangeError::NotFindData),
            },
            _ => Err(ExchangeError::NotFindData),
        }
    })
}

impl Kline {
    pub fn new(
        symbols: &[String],
        s: &SETTINGS_EXCH,
    ) -> impl Future<Output = Result<(Self, Response<Option<Vec<u8>>>), ExchangeError>> {
        async move {
            let (conn, resp) = BybitConnection::new(
                &format!("{WS_PUBLIC}/{}", s.category),
                "kline",
                symbols,
                true,
                s,
            )
            .await?;
            Ok((
                Self {
                    conn,
                    retry_or_timeout: RetryOrTimeout {
                        timeout: s.timeout_cycle_ms,
                    },
                },
                resp,
            ))
        }
    }
}

impl KlineTrait for Kline {
    fn run(&self) -> impl Future<Output = Result<ResultWrap<Vec<f64>>, ExchangeError>> {
        async move {
            self.retry_or_timeout
                .run(async || next(&self.conn.0).await)
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bybit::prelude_tests::prelude::*;
    static SYMBOLS: LazyLock<Vec<String>> = LazyLock::new(|| {
        vec![
            "BTCUSDT".to_string(),
            "ETHUSDT".to_string(),
            "SUIUSDT".to_string(),
            "1INCHUSDT".to_string(),
        ]
    });

    #[tokio::test]
    async fn connect_res_1() {
        let (_, resp) = Kline::new(&SYMBOLS, &S).await.unwrap();
        assert_eq_pr!(resp.status().to_string(), "101 Switching Protocols")
    }

    #[tokio::test]
    async fn run_res_1() {
        let (kline, _) = Kline::new(&SYMBOLS, &S).await.unwrap();
        assert!(!kline.run().await.unwrap().res.is_empty());
    }
}
