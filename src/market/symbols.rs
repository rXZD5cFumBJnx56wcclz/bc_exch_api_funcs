use crate::prelude::*;

pub trait SymbolsTrait {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
    ) -> impl Future<Output = Result<Wrap<Vec<String>>, ExchangeError>>;
}
