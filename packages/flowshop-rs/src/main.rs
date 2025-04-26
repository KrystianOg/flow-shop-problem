mod neh;
mod reader;

use neh::{calculate_cmax, neh};
use reader::read_benchmarks;
use std::time::Instant;

fn main() {
    let benchmarks = read_benchmarks("taillard-benchmark/tai500_20.txt");

    for (i, bench) in benchmarks.iter().enumerate() {
        println!("Benchamrk {}", i + 1);

        let start = Instant::now();
        let order = neh(&bench.times);
        let duration = start.elapsed();

        println!("Time: {}ms", duration.as_secs_f64());
        println!("Order: {:?}", order);

        let cmax = calculate_cmax(&order, &bench.times);

        println!("Cmax (makespan): {}, LB: {}", cmax, &bench.lower_bound);
    }
}
