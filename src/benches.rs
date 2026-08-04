use std::fmt::Display;
use std::io::{self, Write};

use crate::prelude::*;

#[derive(Debug)]
pub struct BenchStatisticsOne {
    pub avg: f64,
    pub median: f64,
    pub min: f64,
    pub max: f64,
    pub res: Vec<Duration>,
}

impl Display for BenchStatisticsOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\
            avg: {} sec\n\
            median: {} sec\n\
            min: {} sec\n\
            max: {} sec\n\
            ",
            self.avg, self.median, self.min, self.max,
        )?;
        Ok(())
    }
}

pub struct BenchStatisticsFull {
    pub connection: BenchStatisticsOne,
    pub send: BenchStatisticsOne,
    pub ping: BenchStatisticsOne,
}

impl Display for BenchStatisticsFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "connection:\n{}\nsend:\n{}\n\nping:\n{}", self.connection, self.send, self.ping)
    }
}

pub fn stat_bench(res: &[Duration]) -> BenchStatisticsOne {
    let mut sorted = res.to_vec();
    sorted.sort();
    BenchStatisticsOne {
        avg: sorted.iter().map(|v| v.as_secs_f64()).sum::<f64>() / sorted.len() as f64,
        median: sorted[sorted.len() / 2].as_secs_f64(),
        min: sorted.iter().min().unwrap().as_secs_f64(),
        max: sorted.iter().max().unwrap().as_secs_f64(),
        res: sorted,
    }
}

pub async fn cycle_identifier<T>(
    time_reference_sec: f64,
    time_out: f64,
    func: &impl AsyncFn() -> Result<T, ExchangeError>,
) -> Result<usize, ExchangeError> {
    let inst1 = Instant::now();
    let mut res = Vec::new();
    for cycle in 1usize..999999999 {
        io::stdout().flush().unwrap();
        print!("\rcycle_identifier..{cycle}");
        let instcycle1 = Instant::now();
        func().await?;
        res.push(Instant::now().duration_since(instcycle1));
        if Instant::now().duration_since(inst1).as_secs_f64() >= time_out {
            break;
        }
    }
    println!();
    Ok((time_reference_sec / stat_bench(&res).avg) as usize)
}

pub async fn bench<T>(
    connection: &mut Vec<Duration>,
    send: &mut Vec<Duration>,
    ping: &mut Vec<Duration>,
    func: &impl AsyncFn() -> Result<T, ExchangeError>,
    func_ping: &impl AsyncFn() -> Result<(), ExchangeError>,
    func_server_time: &impl Fn(T) -> Duration,
) ->  Result<(), ExchangeError> {
    let time1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let t = func_server_time(func().await?);
    let time2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    func_ping().await?;
    ping.push(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().abs_diff(t));
    send.push(time2.abs_diff(t));
    connection.push(time2 - time1);
    Ok(())
}

pub async fn bench_ws<T>(
    bench_id: &str,
    time_reference_sec: f64,
    time_out_cycle_identifier: f64,
    func: &impl AsyncFn() -> Result<T, ExchangeError>,
    func_ping: &impl AsyncFn() -> Result<(), ExchangeError>,
    func_server_time: &impl Fn(T) -> Duration,
) -> Result<BenchStatisticsFull, ExchangeError> {
    println!("{bench_id}");
    let cycles = cycle_identifier( time_reference_sec, time_out_cycle_identifier, func,).await?;
    println!("cycles: {cycles}");
    let mut connection = Vec::with_capacity(cycles);
    let mut send = Vec::with_capacity(cycles);
    let mut ping = Vec::with_capacity(cycles);
    for cycle in 0..cycles {
        io::stdout().flush().unwrap();
        print!("\rcycle: {cycle}");
        bench(&mut connection, &mut send, &mut ping, func, func_ping, func_server_time).await?;
    }
    println!();
    Ok(BenchStatisticsFull {
        connection: stat_bench(&connection),
        send: stat_bench(&send),
        ping: stat_bench(&ping),
    })
}

