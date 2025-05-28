use criterion::{criterion_group, criterion_main, Criterion};
use flowshop_rs::reader::read_benchmarks;
use flowshop_rs::{iterated_greedy, neh};

fn flatten(times: Vec<Vec<usize>>) -> Vec<usize> {
    times.into_iter().flatten().collect()
}

fn benchmark_ig_versions(c: &mut Criterion) {
    let datasets = [
        ("small", "taillard-benchmark/tai20_5.txt"),
        ("medium", "taillard-benchmark/tai100_10.txt"),
        // ("large", "taillard-benchmark/tai500_20.txt"),
        // ("large-2", "taillard-benchmark/tai500_60.txt"),
        // ("very_large", "taillard-benchmark/tai1000_50.txt"),
    ];

    let ig_versions = [("basic_ig", iterated_greedy::iterated_greedy)];

    for (size_label, path) in datasets {
        let benchmarks = read_benchmarks(path);

        for (bench_idx, bench) in benchmarks.iter().enumerate() {
            let flat_times = flatten(bench.times.clone());
            let optimal_makespan = bench.lower_bound;

            for (algoname, algo_fn) in ig_versions.iter() {
                let bench_id = format!("{size_label}_Bench{}_{}", bench_idx + 1, algoname);
                let flat_times = flat_times.clone();

                let mut makespan: usize = 0;
                let mut error: f64 = 0.0;

                c.bench_function(&bench_id, |b| {
                    b.iter(|| {
                        let order =
                            algo_fn(&flat_times, bench.num_jobs, bench.num_machines, 100, 5);

                        makespan = neh::calculate_cmax(&order, &flat_times, bench.num_machines);

                        error = ((makespan as f64 - optimal_makespan as f64)
                            / optimal_makespan as f64)
                            * 100.0;
                    })
                });

                println!("LB: {optimal_makespan}, Makespan: {makespan}, Error: {error}%");
            }
        }
    }
}

criterion_group!(benches, benchmark_ig_versions);
criterion_main!(benches);
