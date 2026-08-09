#[path = "../prelude.rs"]
mod prelude;
use bc_exch_api_funcs::exchs::bybit::market::tickers::Tickers;
use bc_exch_api_funcs::market::tickers::TickersTrait;
use prelude::*;

#[tokio::main]
async fn main() {
    let tickers = Tickers {
        retry_or_timeout: RetryOrTimeout {
            timeout: S.timeout_cycle_ms,
        },
    };
    println!(
        "{}",
        bench_full(
            "tickers_1".to_string(),
            10.,
            5.,
            &|| tickers.run(&CL, &S, "", ""),
            &|v| v.time,
        )
        .await
        .unwrap()
    );
}
