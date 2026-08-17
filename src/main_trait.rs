use std::{fmt::Debug, time::Duration};

use crate::{
    account::{acc_info::AccInfoTrait, wallet_balance::WalletBalanceTrait},
    market::{
        instr_info::InstrumentsInfoTrait, kline_wws::KlineWwsTrait, klines::KlinesTrait,
        klines_symbols::KlinesSymbolsTrait, symbols::SymbolsTrait, tickers::TickersTrait,
    },
};

#[derive(Debug)]
pub struct Wrap<T> {
    pub topic: Option<String>,
    pub time: Duration,
    pub res: T,
    pub info: Option<String>,
}

pub trait InstrumentsInfoExch {
    type InstrInfoData;
    fn instr_info(&self) -> &impl InstrumentsInfoTrait<Self::InstrInfoData>;
}

pub trait KlineWwsExch {
    fn kline_wws(&self) -> &impl KlineWwsTrait;
}

pub trait KlinesExch {
    fn klines(&self) -> &impl KlinesTrait;
}

pub trait SymbolsExch {
    fn symbols(&self) -> &impl SymbolsTrait;
}

pub trait TickersExch {
    type TickersData;
    fn tickers(&self) -> &impl TickersTrait<Self::TickersData>;
}

pub trait AccInfoExch {
    type AccInfoData;
    fn acc_info(&self) -> &impl AccInfoTrait<Self::AccInfoData>;
}

pub trait WalletBalanceExch {
    type WalletBalanceData;
    fn wallet_balance(&self) -> &impl WalletBalanceTrait<Self::WalletBalanceData>;
}

pub trait KlinesSymbolsExch {
    fn klines_symbols(&self) -> &impl KlinesSymbolsTrait;
}

pub trait Exchange:
    KlinesSymbolsExch
    + KlineWwsExch
    + InstrumentsInfoExch
    + WalletBalanceExch
    + AccInfoExch
    + SymbolsExch
    + TickersExch
{
}
