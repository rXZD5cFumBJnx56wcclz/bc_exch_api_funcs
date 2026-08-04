use tokio_tungstenite::tungstenite::{self, Message};

use crate::{
    bybit::{market::kline::WRAP_KLINE, result_req::BybitMessagePub},
    prelude::*,
};

pub trait Kline {
    fn connect_kline(
        &self,
        s: &SETTINGS_EXCH,
        symbols: &[String],
    ) -> impl Future<Output = Result<Response<Option<Vec<u8>>>, Box<dyn Error>>>;
    fn next_kline_req(&self) -> impl Future<Output = Option<Result<Message, tungstenite::Error>>>;
    fn next_kline(&self) -> impl Future<Output = Result<ResultWrap<Vec<f64>>, ExchangeError>>;
    fn next_kline_a(
        &self,
        s: &SETTINGS_EXCH,
    ) -> impl Future<Output = Result<ResultWrap<Vec<f64>>, ExchangeError>>;
    // fn next_kline_a
}
