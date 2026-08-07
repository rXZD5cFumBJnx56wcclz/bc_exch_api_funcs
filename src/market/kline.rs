use crate::prelude::*;

pub trait Kline {
    fn run(&self) -> impl Future<Output = Result<ResultWrap<Vec<f64>>, ExchangeError>>;
}
