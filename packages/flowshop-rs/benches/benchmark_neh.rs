use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flowshop_rs::neh;
use flowshop_rs::neh_rayon;
use flowshop_rs::reader::read_benchmarks;

type Fn = fn(&[usize], usize, usize) -> Vec<usize>;

fn flatten(times: Vec<Vec<usize>>) -> Vec<usize> {
    times.into_iter().flatten().collect()
}

fn benchmark_neh_versions(c: &mut Criterion) {
    let datasets = [
        // ("small", "taillard-benchmark/tai20_5.txt"),
        // ("medium", "taillard-benchmark/tai100_10.txt"),
        ("large", "taillard-benchmark/tai500_20.txt"),
        // ("large-2", "taillard-benchmark/tai500_60.txt"),
        // ("very_large", "taillard-benchmark/tai1000_50.txt"),
    ];

    let neh_versions: [(&str, Fn); 2] = [
        ("basic_neh", neh::neh),
        ("rayon", neh_rayon::neh),
        // ("improved_neh2", neh2::neh),
        // ("neh3", neh3),
        // ("neh_rayon", neh_rayon),
    ];

    for (size_label, path) in datasets {
        let benchmarks = read_benchmarks(path);

        for (bench_idx, bench) in benchmarks.iter().enumerate() {
            let flat_times = flatten(bench.times.clone());

            for (algoname, algo_fn) in neh_versions.iter() {
                let bench_id = format!("{size_label}_Bench{}_{}", bench_idx + 1, algoname);
                let flat_times = flat_times.clone();

                c.bench_function(&bench_id, |b| {
                    b.iter_batched(
                        || black_box(flat_times.clone()),
                        |flat_times| {
                            let _order = algo_fn(&flat_times, bench.num_jobs, bench.num_machines);
                        },
                        criterion::BatchSize::SmallInput,
                    )
                });
            }
        }
    }
}

criterion_group!(benches, benchmark_neh_versions);
criterion_main!(benches);
