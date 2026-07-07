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
