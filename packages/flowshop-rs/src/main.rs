mod neh;
mod reader;

use neh::{calculate_cmax, neh};
use reader::read_benchmarks;
use std::time::Instant;

fn main() {
    let benchmarks = read_benchmarks("taillard-benchmark/tai500_20.txt");

    // let mut times: Vec<f64> = Vec::new();

    for (i, bench) in benchmarks.iter().enumerate() {
        println!("Benchamrk {}", i + 1);

        let start = Instant::now();
        let order = neh(&bench.times);
        let duration = start.elapsed();

        // times.push(duration.as_secs_f64());

        println!("Time: {}s", duration.as_secs_f64());
        println!("Order: {:?}", order);

        let cmax = calculate_cmax(&order, &bench.times);

        println!("Cmax (makespan): {}, LB: {}", cmax, &bench.lower_bound);
        let diff = cmax as f64 - bench.lower_bound as f64;
        let err = diff as f64 * 100.0 / bench.lower_bound as f64;
        println!("Diff: {}, Error: {}", diff, err);
    }

    // let avg = times.iter().sum() / benchmarks.len();

    // println!("avg time: {}", avg)
}
