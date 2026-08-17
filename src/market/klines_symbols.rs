use crate::prelude::*;

pub trait KlinesSymbolsTrait {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
        symbols: &[String],
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<Wrap<MAP<String, Vec<Vec<f64>>>>, ExchangeError>>;
}
