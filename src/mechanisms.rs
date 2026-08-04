use std::error::Error;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use bc_utils_lg::structs::settings::SETTINGS_EXCH;

use crate::error::ExchangeError;

pub async fn all_or_nothing<T>(
    func: impl AsyncFn() -> Result<T, ExchangeError>,
    s: &SETTINGS_EXCH,
) -> Result<T, ExchangeError> {
    let instant = Instant::now();
    loop {
        if instant.duration_since(Instant::now()) >= s.timeout_cycle_ms {
            return Err(ExchangeError::Timeout);
        }
        if let Ok(res) = func().await {
            return Ok(res);
        }
    }
}

// pub async fn whos_first<T>(
//     func: impl AsyncFn() -> Result<T, ExchangeError>,
//     connected: 
//     keys: &[String],
// ) -> Result<T, ExchangeError> {
//     keys
// }

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    use tokio;


}
