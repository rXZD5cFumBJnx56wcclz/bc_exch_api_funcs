use crate::prelude::*;

pub trait SymbolsTrait {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
    ) -> impl Future<Output = Result<ResultWrap<Vec<String>>, ExchangeError>>;
}

pub trait SymbolsExch<T: SymbolsTrait> {
    fn symbols(&self) -> &T;
}
