use std::cell::RefCell;
use std::error::Error;
use std::time::{Duration, Instant};

use bc_utils_lg::structs::settings::SETTINGS_EXCH;
use futures::SinkExt;
use futures::stream::{SplitSink, SplitStream};
use reqwest::Client;
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;
use tokio_tungstenite::tungstenite::http::Response;
use tokio_tungstenite::tungstenite::{self, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, client_async, connect_async};

use crate::bybit::result_req::OpResponsePub;
use crate::deffunc::usizezero;
use crate::error::ExchangeError;
use crate::main_trait::Exchange;

#[derive(Debug)]
pub struct BYBIT {
    pub rest_client: Client,
    pub sink: Option<SplitSink<WebSocketStream<TlsStream<TcpStream>>, Message>>,
    pub stream_: Option<SplitStream<WebSocketStream<TlsStream<TcpStream>>>>,
    pub last_time: Duration,
}

impl BYBIT {
    pub fn new_rest(s: &SETTINGS_EXCH) -> Self {
        Self {
            rest_client: Client::builder()
                .timeout(Duration::from_millis(usizezero(s.timeout_cycle_ms) as u64))
                .build()
                .unwrap(),
            stream_: None,
            sink: None,
            last_time: Duration::default(),
        }
    }
}

impl BYBIT {
    pub fn ping(&mut self, s: &SETTINGS_EXCH) -> impl Future<Output = Result<(), ExchangeError>> {
        async move {
            let time = Instant::now();
            if time.elapsed() >= self.last_time + Duration::from_millis(s.ping_ms as u64) {
                self.last_time = time.elapsed();
                self.sink
                    .as_mut()
                    .unwrap()
                    .send(Message::Text(
                        r#"{
                    "op": "ping"
                    }"#
                        .into(),
                    ))
                    .await
                    .map_err(|e| ExchangeError::WebSocket(e))?;
            };
            Ok(())
        }
    }
    pub fn pong(&mut self) -> impl Future<Output = Result<(), tungstenite::Error>> {
        async move {
            Ok(self
                .sink
                .as_mut()
                .unwrap()
                .send(Message::Text(
                    r#"{
                    "op": "pong"
                    }"#
                    .into(),
                ))
                .await?)
        }
    }
}

impl Exchange for BYBIT {}
