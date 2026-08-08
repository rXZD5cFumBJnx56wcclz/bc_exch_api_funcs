use crate::{exchs::bybit::market::{kline_wws::Kline, symbols::Symbols, tickers::{TICKERS1, Tickers}}, market::{kline_wws::KlineWwsExch, symbols::SymbolsExch, tickers::TickersExch}};

pub struct Bybit {
    pub kline_wws: Kline,
    pub tickers: Tickers,
    pub symbols: Symbols,
}

impl KlineWwsExch<Kline> for Bybit {
    fn kline_wws(&self) -> &Kline {
        &self.kline_wws
    }
}

impl TickersExch<TICKERS1, Tickers> for Bybit {
    fn tickers(&self) -> &Tickers {
        &self.tickers
    }
}

impl SymbolsExch<Symbols> for Bybit {
    fn symbols(&self) -> &Symbols {
        &self.symbols
    }
}