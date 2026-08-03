use std::fmt::Debug;

use bc_utils_lg::structs::settings::SETTINGS_EXCH;

use crate::{
    // account::{acc_info::AccInfo, wallet_balance::WalletBalance},
    market::{
        kline::Kline,
        klines::Klines,
        // oi::OpenInterest,
        // orderbook::Orderbook,
        src::Src,
        symbols::Symbols,
    },
};

pub trait Exchange: Kline
// Src
// + OpenInterest
// + Orderbook
// + Symbols
// + AccInfo
// + WalletBalance
{
}

#[derive(Debug)]
pub struct ResultWrap<T> {
    pub time: usize,
    pub res: T,
    pub info: Option<String>,
}
