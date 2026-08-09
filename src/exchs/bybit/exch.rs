use crate::{
    account::{acc_info::AccInfoExch, wallet_balance::WalletBalanceExch},
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
        instr_info::InstrumentsInfoExch, kline_wws::KlineWwsExch, klines::KlinesExch,
        symbols::SymbolsExch, tickers::TickersExch,
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
