#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::market::klines::*;
use crate::bybit::prelude::*;

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

impl Src for BYBIT<'_> {
    fn src<'a>(
        &'a self,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<Vec<Vec<f64>>, Box<dyn Error>>> {
        async move {
            let mut res = join_all([self.klines(symbol, limit, start, end)]).await;
            res.remove(0)
            // Ok(zip(res.remove(0)?, res.remove(1)?).map(|(mut v1, v2)| {v1.extend(v2); v1}).collect::<Vec<Vec<f64>>>())
        }
    }
    fn src_a<'a>(
        &'a self,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<Vec<Vec<f64>>, Box<dyn Error>>> {
        async move {
            let mut res = join_all([self.klines_a(symbol, limit, start, end)]).await;
            res.remove(0)
        }
    }
    fn src_series_symbols<'a>(
        &'a self,
        symbols: &'a [String],
    ) -> impl Future<Output = MAP<String, Result<Vec<f64>, Box<dyn Error>>>> {
        async move {
            let mut res = join_all([self.kline_symbols(symbols)]).await;
            res.remove(0)
        }
    }
    fn src_series_symbols_a<'a>(
        &'a self,
        symbols: &'a [String],
    ) -> impl Future<Output = Result<MAP<String, Vec<f64>>, Box<dyn Error>>> {
        async move {
            let mut res = join_all([self.kline_symbols_a(symbols)]).await;
            res.remove(0)
        }
    }
    fn src_series_symbols_ao<'a>(
        &'a self,
        symbols: &'a [String],
    ) -> impl Future<Output = Result<MAP<String, Vec<f64>>, Box<dyn Error>>> {
        async move {
            let mut res = join_all([self.kline_symbols_ao(symbols)]).await;
            res.remove(0)
        }
    }
    fn src_symbols<'a>(
        &'a self,
        symbols: &'a [String],
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = MAP<String, Result<Vec<Vec<f64>>, Box<dyn Error>>>> {
        async move {
            let mut res = join_all([self.klines_symbols(symbols, limit, start, end)]).await;
            res.remove(0)
        }
    }
    fn src_symbols_a<'a>(
        &'a self,
        symbols: &'a [String],
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<MAP<String, Vec<Vec<f64>>>, Box<dyn Error>>> {
        async move {
            let mut res = join_all([self.klines_symbols_a(symbols, limit, start, end)]).await;
            Ok(res.remove(0)?)
        }
    }
}

#[cfg(test)]
mod tests {
    #![warn(unused_must_use)]

    use crate::bybit::market::src::*;

    use crate::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn src_lch_1() {
        EXCH.src("SUIUSDT", 50_000, 0, 0).await.unwrap();
    }

    #[tokio::test]
    async fn src_a_lch_1() {
        EXCH.src_a("SUIUSDT", 50_000, 0, 0).await.unwrap();
    }

    #[tokio::test]
    async fn src_series_symbols_lch_1() {
        EXCH.src_series_symbols(&["SUIUSDT".to_string(), "ETHUSDT".to_string()])
            .await;
    }

    #[tokio::test]
    async fn src_series_symbols_a_lch_1() {
        EXCH.src_series_symbols_a(&["SUIUSDT".to_string(), "ETHUSDT".to_string()])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn src_series_symbols_ao_lch_1() {
        EXCH.src_series_symbols_ao(&["SUIUSDT".to_string(), "ETHUSDT".to_string()])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn src_symbols_lch_1() {
        EXCH.src_symbols(
            &["SUIUSDT".to_string(), "ETHUSDT".to_string()],
            50_000,
            0,
            0,
        )
        .await;
    }

    #[tokio::test]
    async fn src_symbols_a_lch_1() {
        EXCH.src_symbols_a(
            &["SUIUSDT".to_string(), "ETHUSDT".to_string()],
            50_000,
            0,
            0,
        )
        .await
        .unwrap();
    }
}
