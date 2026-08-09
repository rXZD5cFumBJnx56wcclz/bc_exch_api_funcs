#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::exchs::bybit::prelude::*;

pub trait AccInfoTrait<T> {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
    ) -> impl Future<Output = Result<ResultWrap<T>, ExchangeError>>;
}
