#![allow(non_camel_case_types)]

use crate::{bybit::prelude::*, market::kline::Kline as KlineTrait};

pub struct Kline {
    pub conn: Mutex<BybitConnections>,
    pub retry_or_timeout: RetryOrTimeout,
    pub race: Race<Result<ResultWrap<Vec<f64>>, ExchangeError>>,
}

impl Default for Kline {
    fn default() -> Self {
        Self {
            conn: Default::default(),
            retry_or_timeout: Default::default(),
            race: Race {
                futures: Default::default(),
            },
        }
    }
}

impl Kline {
    pub fn new(
        symbols: Vec<&[String]>,
        s: &SETTINGS_EXCH,
    ) -> impl Future<Output = Result<(Self, Vec<Response<Option<Vec<u8>>>>), ExchangeError>> {
        async move {
            let mut b = Self::default();
            b.retry_or_timeout.timeout = s.timeout_cycle_ms;
            let (conn, resp) = BybitConnections::new(
                &format!("{WS_PUBLIC}/{}", s.category),
                "kline",
                symbols,
                true,
                s,
            )
            .await?;
            b.conn = Mutex::new(conn);
            Ok((b, resp))
        }
    }
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

fn next(conn: Arc<Connection>) -> BoxFuture<'static, Result<ResultWrap<Vec<f64>>, ExchangeError>> {
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

impl KlineTrait for Kline {
    fn run(&self) -> impl Future<Output = Result<ResultWrap<Vec<f64>>, ExchangeError>> {
        async move {
            self.retry_or_timeout
                .run(async || self.race.run(next, &self.conn.lock().await.0.0).await)
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bybit::prelude_tests::prelude::*;
    static SYMBOLS: LazyLock<Vec<String>> = LazyLock::new(|| vec![
            "BTCUSDT".to_string(),
            "ETHUSDT".to_string(),
            "SUIUSDT".to_string(),
            "1INCHUSDT".to_string(),
        ]);

    #[tokio::test]
    async fn connect_res_1() {
        let symbols = symbols_splitted(&SYMBOLS, 2);
        let (_, resp) = Kline::new(symbols, &S).await.unwrap();
        for res in resp.iter() {
            assert_eq_pr!(res.status().to_string(), "101 Switching Protocols");
        }
        assert_eq_pr!(resp.len(), 2);
    }

    #[tokio::test]
    async fn run_res_1() {
        let symbols = symbols_splitted(&SYMBOLS, 2);
        let (kline, _) = Kline::new(symbols, &S).await.unwrap();
        assert!(!kline.run().await.unwrap().res.is_empty());
    }

    #[tokio::test]
    async fn ping_res_1() {
        let symbols = symbols_splitted(&SYMBOLS, 2);
        let (kline, _) = Kline::new(symbols, &S).await.unwrap();
        kline.conn.lock().await.ping().await.unwrap();
    }
}
