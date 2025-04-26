mod neh;
mod neh2;
mod reader;

// use neh::{calculate_cmax, neh};
use neh2::nehh;
use reader::read_benchmarks;
use std::time::Instant;

fn flatten(times: Vec<Vec<usize>>) -> Vec<usize> {
    times.into_iter().flatten().collect()
}

fn main() {
    let benchmarks = read_benchmarks("taillard-benchmark/tai500_20.txt");

    // let mut times: Vec<f64> = Vec::new();

    for (i, bench) in benchmarks.iter().enumerate() {
        println!("Benchamrk {}", i + 1);

        // let mut start = Instant::now();
        // let order = neh(&bench.times);
        // let duration = start.elapsed();

        let start = Instant::now();

        let times = flatten(bench.times.clone());
        let order2 = nehh(&times, bench.num_jobs, bench.num_machines);
        let duration2 = start.elapsed();

        // println!("Time: {}s", duration.as_secs_f64());
        println!("Time2: {}s", duration2.as_secs_f64());
        println!("Order: {:?}", order2);

        // let cmax = calculate_cmax(&order2, &bench.times);

        // println!("Cmax (makespan): {}, LB: {}", cmax, &bench.lower_bound);
        // let diff = cmax as f64 - bench.lower_bound as f64;
        // let rpd = diff as f64 * 100.0 / bench.lower_bound as f64;
        // println!("Diff: {}, Error: {}", diff, rpd);
    }

    // let avg = times.iter().sum() / benchmarks.len();

    // println!("avg time: {}", avg)
}
