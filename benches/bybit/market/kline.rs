#[path = "../prelude.rs"]
mod prelude;
use bc_exch_api_funcs::exchs::bybit::market::kline_wws::Kline;
use bc_exch_api_funcs::exchs::bybit::market::symbols::Symbols;
use bc_exch_api_funcs::market::kline_wws::KlineWwsTrait;
use bc_exch_api_funcs::market::symbols::SymbolsTrait;
use prelude::*;

#[tokio::main]
async fn main() {
    let (kline, resp) = Kline::new(&Symbols::new(&S).run(&CL, &S).await.unwrap().res, &S)
        .await
        .unwrap();
    println!(
        "{}",
        bench_full(
            "kline_1".to_string(),
            10.,
            5.,
            &|| kline.run(),
            &|v| v.time,
            &async || ping(&mut *kline.conn.lock().await).await,
            &S
        )
        .await
        .unwrap()
    );
}
