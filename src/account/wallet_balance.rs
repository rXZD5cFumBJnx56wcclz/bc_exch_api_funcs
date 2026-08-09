#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::exchs::bybit::prelude::*;

pub trait WalletBalanceTrait<T> {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
        // optional
        coin: &str,
    ) -> impl Future<Output = Result<ResultWrap<Vec<T>>, ExchangeError>>;
}
