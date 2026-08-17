#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::prelude::*;

pub trait TickersTrait<T> {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
        base_coin: &str,
        exp_date: &str,
    ) -> impl Future<Output = Result<Wrap<Vec<T>>, ExchangeError>>;
}
