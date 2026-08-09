use crate::{
    account::{acc_info::AccInfoTrait, wallet_balance::WalletBalanceTrait},
    exchs::bybit::{
        account::{
            acc_info::{ACC_INFO, AccInfo},
            wallet_balance::{WALLET_BALANCE, WalletBalance},
        },
        market::{
            instr_info::{INSTR_INFO1, InstrInfo},
            kline_wws::Kline,
            klines::Klines,
            symbols::Symbols,
            tickers::{TICKERS1, Tickers},
        },
    },
    market::{
        instr_info::InstrumentsInfoTrait, kline_wws::KlineWwsTrait, klines::KlinesTrait,
        symbols::SymbolsTrait, tickers::TickersTrait,
    },
};

pub struct Bybit {
    pub kline_wws: Kline,
    pub tickers: Tickers,
    pub symbols: Symbols,
    pub klines: Klines,
    pub instr_info: InstrInfo,
    pub acc_info: AccInfo,
    pub wallet_balance: WalletBalance,
}

pub trait InstrumentsInfoExch<Res, T: InstrumentsInfoTrait<Res>> {
    fn instr_info(&self) -> &T;
}

pub trait KlineWwsExch<T: KlineWwsTrait> {
    fn kline_wws(&self) -> &T;
}

pub trait KlinesExch<T: KlinesTrait> {
    fn klines(&self) -> &T;
}

pub trait SymbolsExch<T: SymbolsTrait> {
    fn symbols(&self) -> &T;
}

pub trait TickersExch<Res, T: TickersTrait<Res>> {
    fn tickers(&self) -> &T;
}

pub trait AccInfoExch<Res, T: AccInfoTrait<Res>> {
    fn acc_info(&self) -> &T;
}

pub trait WalletBalanceExch<Res, T: WalletBalanceTrait<Res>> {
    fn wallet_balance(&self) -> &T;
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

impl KlinesExch<Klines> for Bybit {
    fn klines(&self) -> &Klines {
        &self.klines
    }
}

impl InstrumentsInfoExch<INSTR_INFO1, InstrInfo> for Bybit {
    fn instr_info(&self) -> &InstrInfo {
        &self.instr_info
    }
}

impl AccInfoExch<ACC_INFO, AccInfo> for Bybit {
    fn acc_info(&self) -> &AccInfo {
        &self.acc_info
    }
}

impl WalletBalanceExch<WALLET_BALANCE, WalletBalance> for Bybit {
    fn wallet_balance(&self) -> &WalletBalance {
        &self.wallet_balance
    }
}
