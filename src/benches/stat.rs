use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug)]
pub struct BenchStatistics {
    pub avg: f64,
    pub median: f64,
    pub min: f64,
    pub max: f64,
    pub res: Vec<Duration>,
}

impl Display for BenchStatistics {
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

pub struct BenchInfo {
    pub bench_id: String,
    pub iter_cycles: usize,
    pub stats: MAP<String, BenchStatistics>,
}

impl BenchInfo {
    pub fn new(bench_id: String, iter_cycles: usize, stats: MAP<String, BenchStatistics>) -> Self {
        Self {
            bench_id,
            iter_cycles,
            stats,
        }
    }
}

impl Display for BenchInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "bench_id: {}\niter cycles: {}\n",
            self.bench_id, self.iter_cycles,
        )?;
        for (k, stat) in &self.stats {
            write!(f, "{k}: {stat}\n")?;
        }
        Ok(())
    }
}

pub fn stat_bench(res: &[Duration]) -> BenchStatistics {
    let mut sorted = res.to_vec();
    sorted.sort();
    BenchStatistics {
        avg: sorted.iter().map(|v| v.as_secs_f64()).sum::<f64>() / sorted.len() as f64,
        median: sorted[sorted.len() / 2].as_secs_f64(),
        min: sorted.iter().min().unwrap().as_secs_f64(),
        max: sorted.iter().max().unwrap().as_secs_f64(),
        res: sorted,
    }
}
