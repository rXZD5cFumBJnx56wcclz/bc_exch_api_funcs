#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::prelude::*;
pub use crate::market::instr_info::*;

pub const INSTR_INFO: &str = "/v5/market/instruments-info";

ResultWrap<INSTR_INFO> for RESULT_EXCH_BYBIT<INSTR_INFO> {
    fn res(self) -> INSTR_INFO {
        self.result
    }
}

impl InstrumentsInfo for BYBIT {
    fn instr_info_req(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        status: &str,
        base_coin: &str,
        limit: usize,
        cursor: &str,
    ) -> impl Future<Output = Result<ResultWrap<INSTR_INFO>, Error_req>> {
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
                    &s.url, &s.category,
                ))
                .send()
                .await?
                .json::<RESULT_EXCH_BYBIT<INSTR_INFO>>()
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bybit::market::instr_info::*;
    use crate::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn instr_info_req_lch_1() {
        EXCH()
            .instr_info_req(&S, "BTCUSDT", "", "", 1, "")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn instr_info_lch_1() {
        EXCH().instr_info(&S, "SUIUSDT", "", "").await.unwrap();
    }

    #[tokio::test]
    async fn instrs_info_lch_1() {
        EXCH()
            .instrs_info(
                &S,
                &[
                    "SUIUSDT".to_string(),
                    "UNIUSDT".to_string(),
                    "ETHUSDT".to_string(),
                ],
                "",
                "",
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn instrs_info_a_lch_1() {
        EXCH()
            .instrs_info_a(
                &S,
                &[
                    "SUIUSDT".to_string(),
                    "UNIUSDT".to_string(),
                    "ETHUSDT".to_string(),
                ],
                "",
                "",
            )
            .await
            .unwrap();
    }
}
