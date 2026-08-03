#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{market::symbols::Symbols, prelude::*};

pub trait Klines: Symbols {
    fn klines(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<ResultWrap<Vec<Vec<f64>>>, ExchangeError>>;
    fn klines_a(
        &self,
        s: &SETTINGS_EXCH,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<ResultWrap<Vec<Vec<f64>>>, ExchangeError>> {
        async move {
            all_or_nothing(
                async || Ok(self.klines(s, symbol, limit, start, end).await?),
                s,
            )
            .await
        }
    }

    fn klines_symbols(
        &self,
        s: &SETTINGS_EXCH,
        symbols: &[String],
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<MAP<String, ResultWrap<Vec<Vec<f64>>>>, ExchangeError>> {
        async move {
            join_all(symbols.iter().map(|symbol| async {
                Ok((
                    symbol.clone(),
                    self.klines(s, symbol, limit, start, end).await?,
                ))
            }))
            .await
            .into_iter()
            .collect()
        }
    }

    fn klines_symbols_a(
        &self,
        s: &SETTINGS_EXCH,
        symbols: &[String],
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<MAP<String, ResultWrap<Vec<Vec<f64>>>>, ExchangeError>> {
        async move {
            join_all(symbols.iter().map(|symbol| async {
                Ok((
                    symbol.clone(),
                    self.klines_a(s, symbol, limit, start, end).await?,
                ))
            }))
            .await
            .into_iter()
            .collect::<Result<_, ExchangeError>>()
        }
    }
}
