#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::market::instr_info::*;
use crate::bybit::prelude::*;

impl InstrumentsInfo for BYBIT<'_> {
    fn instr_info_req<'a, T>(
        &'a self,
        symbol: &str,
        status: &str,
        base_coin: &str,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<impl ResultWrap<RESULT_WRAP_INSTR_INFO>, Error_req>>
    where 
        T: ResultWrap<Vec<INSTR_INFO1>>
    {
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
                .json::<RESULT_EXCH_BYBIT<RESULT_WRAP_INSTR_INFO>>()
                .await
        }
    }
}
