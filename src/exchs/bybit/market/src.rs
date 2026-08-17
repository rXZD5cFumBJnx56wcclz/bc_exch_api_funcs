#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::exchs::bybit::market::klines::*;
use crate::exchs::bybit::prelude::*;

pub trait Src: Exchange + Kline {
    fn src<'a>(
        &'a self,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<Vec<Vec<f64>>, Box<dyn Error>>>;
    fn src_a<'a>(
        &'a self,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<Vec<Vec<f64>>, Box<dyn Error>>>;
    fn src_series_symbols<'a>(
        &'a self,
        symbols: &'a [String],
    ) -> impl Future<Output = MAP<String, Result<Vec<f64>, Box<dyn Error>>>>;
    fn src_series_symbols_a<'a>(
        &'a self,
        symbols: &'a [String],
    ) -> impl Future<Output = Result<MAP<String, Vec<f64>>, Box<dyn Error>>>;
    fn src_series_symbols_ao<'a>(
        &'a self,
        symbols: &'a [String],
    ) -> impl Future<Output = Result<MAP<String, Vec<f64>>, Box<dyn Error>>>;
    fn src_symbols<'a>(
        &'a self,
        symbols: &'a [String],
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = MAP<String, Result<Vec<Vec<f64>>, Box<dyn Error>>>>;
    fn src_symbols_a<'a>(
        &'a self,
        symbols: &'a [String],
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<MAP<String, Vec<Vec<f64>>>, Box<dyn Error>>>;
}

impl Src for BYBIT {
    fn src_a(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<Wrap<Vec<Vec<f64>>>, ExchangeError>> {
        async move {
            let mut res = join_all([self.klines_a(s, symbol, limit, start, end)]).await;
            res.remove(0)
            // Ok(zip(res.remove(0)?, res.remove(1)?).map(|(mut v1, v2)| {v1.extend(v2); v1}).collect::<Vec<Vec<f64>>>())
        }
    }
    fn src_symbols_a(
        &self,
        s: &SETTINGS_EXCH,
        symbols: &[String],
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<MAP<String, Wrap<Vec<Vec<f64>>>>, ExchangeError>> {
        async move {
            let mut res = join_all([self.klines_symbols_a(s, symbols, limit, start, end)]).await;
            res.remove(0)
        }
    }
}

#[cfg(test)]
mod tests {
    #![warn(unused_must_use)]

    use crate::exchs::bybit::market::src::*;

    use crate::exchs::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn src_symbols_a_lch_1() {
        EXCH()
            .src_symbols_a(
                &S,
                &["SUIUSDT".to_string(), "ETHUSDT".to_string()],
                1000,
                0,
                0,
            )
            .await
            .unwrap();
    }
}
