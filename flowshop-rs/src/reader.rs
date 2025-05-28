use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Benchmark {
    pub num_jobs: usize,
    pub num_machines: usize,
    pub lower_bound: usize,
    pub times: Vec<Vec<usize>>, // times[job][machine]
}

pub fn read_benchmarks(path: &str) -> Vec<Benchmark> {
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut benchmarks = Vec::new();
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        if line.trim().starts_with("number of jobs") {
            let header = line;
            let mut parts = header
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>();

            while parts.len() < 5 {
                if let Some(Ok(next_line)) = lines.next() {
                    parts.extend(
                        next_line
                            .split_whitespace()
                            .filter_map(|s| s.parse::<usize>().ok()),
                    );
                }
            }

            let num_jobs = parts[0];
            let num_machines = parts[1];
            let lower_bound = parts[4];
            // ignore seed, upper bound, lower bound for now

            // Expect "processing times :" line
            while let Some(Ok(line)) = lines.next() {
                if line.trim().starts_with("processing times") {
                    break;
                }
            }

            let mut times = vec![vec![0; num_machines]; num_jobs];
            for machine in 0..num_machines {
                if let Some(Ok(line)) = lines.next() {
                    let numbers = line
                        .split_whitespace()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect::<Vec<_>>();

                    for (job, &time) in numbers.iter().enumerate() {
                        times[job][machine] = time;
                    }
                }
            }

            benchmarks.push(Benchmark {
                num_jobs,
                num_machines,
                lower_bound,
                times,
            });
        }
    }

    benchmarks
}
