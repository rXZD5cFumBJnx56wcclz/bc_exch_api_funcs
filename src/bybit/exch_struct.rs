use std::cell::RefCell;
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};

use bc_utils_lg::structs::settings::SETTINGS_EXCH;
use bc_utils_lg::types::maps::MAP;
use futures::SinkExt;
use futures::stream::{SplitSink, SplitStream};
use reqwest::Client;
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;
use tokio_tungstenite::tungstenite::http::Response;
use tokio_tungstenite::tungstenite::{self, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, client_async, connect_async};
use tokio::sync::Mutex;

use crate::bybit::result_req::OpResponsePub;
use crate::error::ExchangeError;
use crate::main_trait::Exchange;
use crate::ws_connect::*;

#[derive(Debug)]
pub struct BYBIT {
    pub rest_client: Client,
    pub wws_connected: Arc<Mutex<MAP<String, (WwsSink, WwsStream)>>>,
    pub last_time: RefCell<Duration>,
}

impl BYBIT {
    pub fn new_rest(s: &SETTINGS_EXCH) -> Self {
        Self {
            rest_client: Client::builder()
                .timeout(s.timeout_cycle_ms)
                .build()
                .unwrap(),
            wws_connected: Default::default(),
            last_time: Default::default(),
        }
    }
}

impl BYBIT {
    pub fn sink_msg(&self, msg: &Message) -> impl Future<Output = Result<(), ExchangeError>> {
        async move {
            for sink in self.wws_connected.lock().await.values_mut() {
                sink.0.send(msg.clone())
                    .await
                    .map_err(|e| ExchangeError::WebSocket(e))?;
            }
            Ok(())
        }
    }
    pub fn ping(&self, s: &SETTINGS_EXCH) -> impl Future<Output = Result<(), ExchangeError>> {
        async move {
            let time = Instant::now();
            if time.elapsed() >= *self.last_time.borrow() + s.ping_ms {
                *self.last_time.borrow_mut() = time.elapsed();
                self.sink_msg(&Message::Text(
                    r#"{
                    "op": "ping"
                    }"#
                    .into(),
                ))
                .await?;
            }
            Ok(())
        }
    }
    pub fn pong(&self) -> impl Future<Output = Result<(), ExchangeError>> {
        async move {
            self.sink_msg(&Message::Text(
                r#"{
                    "op": "pong"
                    }"#
                .into(),
            ))
            .await
        }
    }
}

impl Exchange for BYBIT {}
