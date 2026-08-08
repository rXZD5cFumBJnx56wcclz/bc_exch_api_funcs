use crate::{
    exchs::bybit::{market::tickers::Tickers, prelude::*},
    market::{symbols::SymbolsTrait, tickers::TickersTrait},
};

pub struct Symbols {
    pub tickers: Tickers,
}

impl Symbols {
    pub fn new(s: &SETTINGS_EXCH) -> Self {
        Self {
            tickers: Tickers::new(s),
        }
    }
}

impl SymbolsTrait for Symbols {
    fn run(
        &self,
        cl: &Client,
        s: &SETTINGS_EXCH,
    ) -> impl Future<Output = Result<ResultWrap<Vec<String>>, ExchangeError>> {
        async move {
            let res = self.tickers.run(cl, s, "", "").await?;
            Ok(ResultWrap {
                time: res.time,
                res: res.res.into_iter().map(|v| v.symbol).collect(),
                topic: None,
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
    async fn symbols_res_1() {
        assert!(!Symbols::new(&S).run(&CL, &S).await.unwrap().res.is_empty())
    }
}
