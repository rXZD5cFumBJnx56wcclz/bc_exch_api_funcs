use crate::prelude::*;

pub async fn bench_iter_func<T>(
    func: &impl AsyncFn() -> Result<T, ExchangeError>,
    vector: &mut Vec<Duration>,
) -> Result<(), ExchangeError> {
    let instcycle1 = Instant::now();
    func().await?;
    vector.push(Instant::now().duration_since(instcycle1));
    Ok(())
}

pub async fn bench<T>(
    cycles: usize,
    func: &impl AsyncFn() -> Result<T, ExchangeError>,
) -> Result<MAP<String, Vec<Duration>>, ExchangeError> {
    let mut vector = Vec::with_capacity(cycles);
    for _ in 0..cycles {
        bench_iter_func(func, &mut vector).await?;
    }
    Ok(MAP::from_iter([("send".to_string(), vector)]))
}

pub async fn bench_all_iter<T>(
    send: &mut Vec<Duration>,
    send_server_time: &mut Vec<Duration>,
    send_server_time_ping: &mut Vec<Duration>,
    func: &impl AsyncFn() -> Result<T, ExchangeError>,
    func_ping: &impl AsyncFn() -> Result<(), ExchangeError>,
    ping_time_bf: &mut Duration,
    func_server_time: &impl Fn(T) -> Duration,
    s: &SETTINGS_EXCH,
) -> Result<(), ExchangeError> {
    let time1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let t = func_server_time(func().await?);
    let time2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    if time2 - *ping_time_bf >= s.ping_ms {
        func_ping().await?;
        *ping_time_bf = time2;
    }
    send_server_time_ping.push(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .abs_diff(t),
    );
    send.push(time2 - time1);
    send_server_time.push(time2.abs_diff(t));
    Ok(())
}

pub async fn bench_all<T>(
    cycles: usize,
    func: &impl AsyncFn() -> Result<T, ExchangeError>,
    func_server_time: &impl Fn(T) -> Duration,
    func_ping: &impl AsyncFn() -> Result<(), ExchangeError>,
    s: &SETTINGS_EXCH,
) -> Result<MAP<String, Vec<Duration>>, ExchangeError> {
    let mut send = Vec::with_capacity(cycles);
    let mut send_server_time = Vec::with_capacity(cycles);
    let mut send_server_time_ping = Vec::with_capacity(cycles);
    let mut time_ping = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..cycles {
        bench_all_iter(
            &mut send,
            &mut send_server_time,
            &mut send_server_time_ping,
            func,
            func_ping,
            &mut time_ping,
            func_server_time,
            s,
        )
        .await?;
    }
    Ok(MAP::from_iter([
        ("send".to_string(), send),
        ("send_server_time".to_string(), send_server_time),
        ("send_server_time_ping".to_string(), send_server_time_ping),
    ]))
}

pub async fn bench_full<T>(
    bench_id: String,
    time_reference_sec: f64,
    time_out: f64,
    func: &impl AsyncFn() -> Result<T, ExchangeError>,
    func_server_time: &impl Fn(T) -> Duration,
    func_ping: &impl AsyncFn() -> Result<(), ExchangeError>,
    s: &SETTINGS_EXCH,
) -> Result<BenchInfo, ExchangeError> {
    let cycles = cycle_identifier(time_reference_sec, time_out, func).await?;
    Ok(BenchInfo::new(
        bench_id,
        bench_all(cycles, func, func_server_time, func_ping, s).await?,
    ))
}
