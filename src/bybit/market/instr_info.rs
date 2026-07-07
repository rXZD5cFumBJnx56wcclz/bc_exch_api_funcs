#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::prelude::*;
pub use crate::market::instr_info::*;

pub const INSTR_INFO: &str = "/v5/market/instruments-info";

impl ResultWrap<INSTR_INFO> for RESULT_EXCH_BYBIT<INSTR_INFO> {
    fn res(self) -> INSTR_INFO {
        self.result
    }
}

impl InstrumentsInfo for BYBIT<'_> {
    fn instr_info_req<'a>(
        &'a self,
        symbol: &str,
        status: &str,
        base_coin: &str,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<impl ResultWrap<INSTR_INFO>, Error_req>> {
        async move {
            self.client
                .get(format!(
                    "{}{INSTR_INFO}\
                ?category={}\
                &symbol={symbol}\
                &status={status}\
                &baseCoin={base_coin}\
                &limit={limit}\
                &cursor={cursor}",
                    &self.s().exch.url,
                    &self.s().trade.category,
                ))
                .send()
                .await?
                .json::<RESULT_EXCH_BYBIT<INSTR_INFO>>()
                .await
        }
    }
}
