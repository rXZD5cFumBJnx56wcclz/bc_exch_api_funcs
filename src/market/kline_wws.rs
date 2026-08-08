use crate::prelude::*;

pub trait KlineWwsTrait {
    fn run(&self) -> impl Future<Output = Result<ResultWrap<Vec<f64>>, ExchangeError>>;
}
