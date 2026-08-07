use crate::benches::bench::*;
use crate::benches::stat::*;
use crate::prelude::*;

pub async fn cycle_identifier<T>(
    time_reference_sec: f64,
    time_out: f64,
    func: &impl AsyncFn() -> Result<T, ExchangeError>,
) -> Result<usize, ExchangeError> {
    let inst1 = Instant::now();
    let mut res = Vec::new();
    for _ in 0usize..usize::MAX {
        bench_iter_func(func, &mut res).await?;
        if Instant::now().duration_since(inst1).as_secs_f64() >= time_out {
            break;
        }
    }
    Ok((time_reference_sec / stat_bench(&res).avg) as usize)
}

pub async fn connections_numbers<T>(
    symbols_num: f64,
    cycles: usize,
    time_out_sec: f64,
    func: &impl AsyncFn() -> Result<T, ExchangeError>,
) -> Result<usize, ExchangeError> {
    Ok((stat_bench(&bench(cycles, func).await?["send"]).avg * symbols_num / time_out_sec) as usize)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[tokio::test]
    async fn connections_numbers_res_1() {
        assert_eq_pr!(
            connections_numbers(700., 100, 10., &async || Ok(tokio::time::sleep(
                Duration::from_millis(200)
            )
            .await))
            .await
            .unwrap(),
            14
        );
    }
}
