#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{market::klines::Klines, prelude::*};

// src: 0 time, 1 open_price, 2 high_price, 3 low_price, 4 close_price, 5 volume, 6 turnover, 7 index, 8 mark,
pub trait Src: Klines {
    fn src_a(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<ResultWrap<Vec<Vec<f64>>>, ExchangeError>>;
    fn src_symbols_a<'a>(
        &self,
        s: &SETTINGS_EXCH,
        symbols: &'a [String],
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<MAP<String, ResultWrap<Vec<Vec<f64>>>>, ExchangeError>>;
}
