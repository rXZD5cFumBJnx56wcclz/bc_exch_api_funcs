#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use bc_utils_lg::structs::settings::SETTINGS_EXCH;

use crate::prelude::*;

pub trait InstrumentsInfoTrait<T> {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
        symbol: &str,
        base_coin: &str,
        limit: usize,
    ) -> impl Future<Output = Result<ResultWrap<MAP<String, T>>, ExchangeError>>;
}

pub trait InstrumentsInfoExch<Res, T: InstrumentsInfoTrait<Res>> {
    fn instr_info(&self) -> &T;
}
