#[path = "../prelude.rs"]
mod prelude;
use bc_exch_api_funcs::account::wallet_balance::WalletBalanceTrait;
use bc_exch_api_funcs::exchs::bybit::account::wallet_balance::WalletBalance;
use prelude::*;

#[tokio::main]
async fn main() {
    let wallet_balance = WalletBalance {
        retry_or_timeout: RetryOrTimeout {
            timeout: S.timeout_cycle_ms,
        },
    };
    println!(
        "{}",
        bench_full(
            "wallet_balance_1".to_string(),
            10.,
            5.,
            &|| wallet_balance.run(&CL, &S, "USDT"),
            &|v| v.time,
        )
        .await
        .unwrap()
    );
}
