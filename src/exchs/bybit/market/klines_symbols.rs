use crate::{
    exchs::bybit::{market::klines::Klines, prelude::*},
    market::{klines::KlinesTrait, klines_symbols::KlinesSymbolsTrait},
};

pub struct KlinesSymbols {
    pub klines: Klines,
    pub join_all: JoinAll,
}

impl KlinesSymbolsTrait for KlinesSymbols {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
        symbols: &[String],
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<ResultWrap<MAP<String, Vec<Vec<f64>>>>, ExchangeError>> {
        async move {
            let res = self
                .join_all
                .run(symbols.iter().map(|v| {
                    (
                        v.clone(),
                        self.klines.run(cl, s, v.as_str(), limit, start, end),
                    )
                }))
                .await;
            Ok(ResultWrap {
                topic: None,
                time: res
                    .values()
                    .map(|v| Ok(v.as_ref().map_err(|_| ExchangeError::NotFindData)?.time))
                    .collect::<Result<Vec<Duration>, ExchangeError>>()?
                    .into_iter()
                    .sum::<Duration>(),
                res: res
                    .into_iter()
                    .map(|(sym, klines)| Ok((sym, klines?.res)))
                    .collect::<Result<MAP<_, _>, ExchangeError>>()?,
                info: None,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::exchs::bybit::prelude_tests::prelude::*;

    #[tokio::test]
    async fn klines_symbols_res_1() {
        assert!(
            !KlinesSymbols {
                klines: Klines {
                    retry_or_timeout: RetryOrTimeout {
                        timeout: S.timeout_cycle_ms
                    }
                },
                join_all: JoinAll
            }
            .run(
                &CL,
                &S,
                &[
                    "SUIUSDT".to_string(),
                    "ETHUSDT".to_string(),
                    "BTCUSDT".to_string(),
                ],
                1000,
                0,
                0
            )
            .await
            .unwrap()
            .res
            .is_empty(),
        );
    }
}
