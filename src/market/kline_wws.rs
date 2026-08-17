use crate::prelude::*;

pub trait KlineWwsTrait {
    fn run(&self) -> impl Future<Output = Result<Wrap<Vec<f64>>, ExchangeError>>;
}
