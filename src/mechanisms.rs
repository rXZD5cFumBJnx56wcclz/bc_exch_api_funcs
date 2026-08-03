use std::error::Error;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use bc_utils_lg::structs::settings::SETTINGS_EXCH;

use crate::error::ExchangeError;

pub async fn all_or_nothing<T, FUT>(
    func: impl Fn() -> FUT,
    s: &SETTINGS_EXCH,
) -> Result<T, ExchangeError>
where
    FUT: Future<Output = Result<T, ExchangeError>>,
{
    let instant = Instant::now();
    loop {
        if instant.duration_since(Instant::now()).as_millis() as usize > s.timeout_cycle_ms {
            return Err(ExchangeError::Timeout);
        }
        if let Ok(res) = func().await {
            return Ok(res);
        }
    }
}

pub async fn check<T, Fut>(
    func: impl Fn() -> Fut,
    check: impl Fn(&T) -> Result<bool, Box<dyn Error>>,
) -> Result<T, Box<dyn Error>>
where
    Fut: Future<Output = Result<T, Box<dyn Error>>>,
{
    loop {
        let res = func().await?;
        if check(&res)? {
            return Ok(res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    use tokio;

    #[tokio::test]
    async fn check_res_1() -> Result<(), Box<dyn Error>> {
        check(
            async || Ok(vec![1, 1]),
            |v| {
                let first = v.first().ok_or(Box::<dyn Error>::from("err"))?;
                Ok(v.iter().all(|el| el == first))
            },
        )
        .await?;
        Ok(())
    }
}
