#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::prelude::*;

pub trait KlinesTrait {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<ResultWrap<Vec<Vec<f64>>>, ExchangeError>>;
}
