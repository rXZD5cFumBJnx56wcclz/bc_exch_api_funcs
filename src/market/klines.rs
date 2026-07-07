#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bybit::const_url::KLINE;
use crate::bybit::prelude::*;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct RESULT_WRAP_KLINE {
    pub symbol: String,
    pub category: String,
    pub list: Vec<Vec<String>>,
}

pub trait Kline: for<'a> Exchange {
    fn klines_req<'b>(
        &'b self,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<impl ResultWrap<ResultWrap<Vec<Vec<String>>>>, Error_req>>;
    /// the function returns values from the beginning of the start to the end (in ascending order)
    /// It's a cumbersome implementation, but I don't want to fuck with it right now.
    fn klines<'b>(
        &'b self,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<Vec<Vec<f64>>, Box<dyn std::error::Error>>> {
        async move {
            let inter = match self.s().trade.timeframe.as_str() {
                "D" => 1440,
                "W" => 10_080,
                "M" => 43_200,
                v => v.parse::<usize>().unwrap(),
            };
            let time_stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as usize;
            let changes = inter * limit * 60 * 1000;
            let step = inter * 1000 * 60 * 1000;
            let mut klines = Vec::with_capacity(limit);
            let (start, end) = if start != 0 && end == 0 {
                (start, start + changes)
            } else if start == 0 && end != 0 {
                (end - changes, end)
            } else {
                (time_stamp - changes, time_stamp)
            };
            let mut futures = Vec::with_capacity(limit);
            for s in (start..end).step_by(step) {
                futures.push(async move {
                    let mut res = self
                        .klines_req(symbol, limit, s, s + step)
                        .await?
                        .result
                        .list;
                    res.reverse();
                    Ok::<Vec<Vec<f64>>, Box<dyn Error>>(
                        res.into_iter()
                            .map(|v| {
                                v.into_iter()
                                    .map(|v1| v1.parse::<f64>().expect("this not a number"))
                                    .collect()
                            })
                            .collect::<Vec<Vec<f64>>>(),
                    )
                });
            }
            for v in join_all(futures).await {
                klines.extend_from_slice(&v?);
            }
            Ok(klines)
        }
    }
    fn klines_a<'b>(
        &'b self,
        symbol: &str,
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<Vec<Vec<f64>>, Box<dyn Error>>> {
        async move {
            all_or_nothing(
                async || self.klines(symbol, limit, start, end).await,
                usizezero(self.s().exch.timeout_cycle_ms),
            )
            .await
        }
    }

    fn kline_symbols<'b>(
        &'b self,
        symbols: &'b [String],
    ) -> impl Future<Output = MAP<String, Result<Vec<f64>, Box<dyn std::error::Error>>>> {
        async move {
            join_all(symbols.iter().map(|s| async {
                (
                    s.clone(),
                    async {
                        self.klines(s, 1, 0, 0)
                            .await?
                            .into_iter()
                            .next()
                            .ok_or(Box::from("not found"))
                    }
                    .await,
                )
            }))
            .await
            .into_iter()
            .collect()
        }
    }

    fn kline_symbols_a<'b>(
        &'b self,
        symbols: &'b [String],
    ) -> impl Future<Output = Result<MAP<String, Vec<f64>>, Box<dyn Error>>> {
        async move {
            join_all(symbols.iter().map(|s| async {
                Ok((
                    s.clone(),
                    async {
                        match self.klines_a(s, 1, 0, 0).await {
                            Ok(v) => Ok(v.into_iter().next().unwrap_or(Default::default())),
                            Err(_) => Err(Box::<dyn Error>::from("klines_a err")),
                        }
                    }
                    .await?,
                ))
            }))
            .await
            .into_iter()
            .collect::<Result<_, Box<dyn Error>>>()
        }
    }

    fn kline_symbols_ao<'b>(
        &'b self,
        symbols: &'b [String],
    ) -> impl Future<Output = Result<MAP<String, Vec<f64>>, Box<dyn Error>>> {
        async move {
            one_time_hm(
                async || self.kline_symbols_a(symbols).await,
                |v| Ok(v.1.get(0).ok_or(Box::<dyn Error>::from("err"))?),
                |v| {
                    Ok(v.iter()
                        .next()
                        .ok_or(Box::<dyn Error>::from("err"))?
                        .1
                        .get(0)
                        .ok_or(Box::<dyn Error>::from("err"))?)
                },
            )
            .await
        }
    }

    fn klines_symbols<'b>(
        &'b self,
        symbols: &'b [String],
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = MAP<String, Result<Vec<Vec<f64>>, Box<dyn std::error::Error>>>> {
        async move {
            join_all(
                symbols
                    .iter()
                    .map(|s| async { (s.clone(), self.klines(s, limit, start, end).await) }),
            )
            .await
            .into_iter()
            .collect()
        }
    }

    fn klines_symbols_a<'b>(
        &'b self,
        symbols: &'b [String],
        limit: usize,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<MAP<String, Vec<Vec<f64>>>, Box<dyn Error>>> {
        async move {
            join_all(
                symbols.iter().map(|s| async {
                    Ok((s.clone(), self.klines_a(s, limit, start, end).await?))
                }),
            )
            .await
            .into_iter()
            .collect::<Result<_, Box<dyn Error>>>()
        }
    }
}
