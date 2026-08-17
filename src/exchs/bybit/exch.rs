use crate::{
    exchs::bybit::{
        account::{
            acc_info::{ACC_INFO, AccInfo},
            wallet_balance::{WALLET_BALANCE, WalletBalance},
        },
        market::{
            instr_info::{INSTR_INFO1, InstrInfo},
            kline_wws::Kline,
            klines::Klines,
            klines_symbols::KlinesSymbols,
            symbols::Symbols,
            tickers::{TICKERS1, Tickers},
        },
    },
    main_trait::{
        AccInfoExch, InstrumentsInfoExch, KlineWwsExch, KlinesExch, KlinesSymbolsExch, SymbolsExch,
        TickersExch, WalletBalanceExch,
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
    pub klines_symbols: KlinesSymbols,
}

impl KlinesSymbolsExch for Bybit {
    fn klines_symbols(&self) -> &impl crate::market::klines_symbols::KlinesSymbolsTrait {
        &self.klines_symbols
    }
}

impl KlineWwsExch for Bybit {
    fn kline_wws(&self) -> &impl crate::market::kline_wws::KlineWwsTrait {
        &self.kline_wws
    }
}

impl TickersExch for Bybit {
    type TickersData = TICKERS1;
    fn tickers(&self) -> &impl crate::market::tickers::TickersTrait<TICKERS1> {
        &self.tickers
    }
}

impl SymbolsExch for Bybit {
    fn symbols(&self) -> &impl crate::market::symbols::SymbolsTrait {
        &self.symbols
    }
}

impl KlinesExch for Bybit {
    fn klines(&self) -> &impl super::market::klines::KlinesTrait {
        &self.klines
    }
}

impl InstrumentsInfoExch for Bybit {
    type InstrInfoData = INSTR_INFO1;
    fn instr_info(&self) -> &impl super::market::instr_info::InstrumentsInfoTrait<INSTR_INFO1> {
        &self.instr_info
    }
}

impl AccInfoExch for Bybit {
    type AccInfoData = ACC_INFO;
    fn acc_info(&self) -> &impl super::account::acc_info::AccInfoTrait<ACC_INFO> {
        &self.acc_info
    }
}

impl WalletBalanceExch for Bybit {
    type WalletBalanceData = WALLET_BALANCE;
    fn wallet_balance(
        &self,
    ) -> &impl super::account::wallet_balance::WalletBalanceTrait<WALLET_BALANCE> {
        &self.wallet_balance
    }
}
