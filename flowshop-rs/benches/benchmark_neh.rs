use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
// use flowshop_rs::neh;
use flowshop_rs::neh_rayon;
use flowshop_rs::reader::read_benchmarks;

type NehFn = fn(&[usize], usize, usize) -> Vec<usize>;
type ChunkedNehFn = fn(&[usize], usize, usize, usize) -> Vec<usize>;

fn flatten(times: Vec<Vec<usize>>) -> Vec<usize> {
    times.into_iter().flatten().collect()
}

fn get_chunk_size(n_jobs: usize) -> usize {
    match n_jobs {
        0..=50 => 10,
        51..=200 => 25,
        201..=500 => 50,
        501..=1000 => 100,
        _ => 150,
    }
}

fn benchmark_neh_versions(c: &mut Criterion) {
    let datasets = [
        // ("small", "taillard-benchmark/tai20_5.txt"),
        // ("medium", "taillard-benchmark/tai100_10.txt"),
        // ("large", "taillard-benchmark/tai500_20.txt"),
        ("large-2", "taillard-benchmark/tai500_60.txt"),
        // ("very_large", "taillard-benchmark/tai1000_50.txt"),
    ];

    let basic_algorithms: Vec<(&str, NehFn)> = vec![
        // ("basic_neh", neh::neh),
        ("parallel", neh_rayon::neh),
        ("parallel_optimized", neh_rayon::neh_parallel_optimized),
    ];

    let chunked_algorithm: (&str, ChunkedNehFn) =
        ("parallel_chunked", neh_rayon::neh_parallel_chunked);

    for (size_label, path) in datasets {
        println!("Loading dataset: {}", path);
        let benchmarks = read_benchmarks(path);

        for (bench_idx, bench) in benchmarks.iter().enumerate() {
            let flat_times = flatten(bench.times.clone());
            let chunk_size = get_chunk_size(bench.num_jobs);

            println!(
                "Benchmark {}: {} jobs x {} machines, chunk_size: {}",
                bench_idx + 1,
                bench.num_jobs,
                bench.num_machines,
                chunk_size
            );

            // Create a benchmark group for this specific problem instance
            let mut group = c.benchmark_group(format!("{}_bench_{}", size_label, bench_idx + 1));

            // Set longer measurement time for more stable results
            group.measurement_time(std::time::Duration::from_secs(10));
            group.sample_size(20);

            for &(algo_name, algo_fn) in &basic_algorithms {
                group.bench_with_input(
                    BenchmarkId::new(
                        algo_name,
                        format!("{}x{}", bench.num_jobs, bench.num_machines),
                    ),
                    &(&flat_times, bench.num_jobs, bench.num_machines),
                    |b, &(times, n_jobs, n_machines)| {
                        b.iter(|| {
                            let _order =
                                algo_fn(black_box(times), black_box(n_jobs), black_box(n_machines));
                        })
                    },
                );

                // let bench_id = format!("{size_label}_Bench{}_{}", bench_idx + 1,);
                // let flat_times = flat_times.clone();
                //
                // c.bench_function(&bench_id, |b| {
                //     b.iter_batched(
                //         || black_box(flat_times.clone()),
                //         |flat_times| {
                //             let _order = algo_fn(&flat_times, bench.num_jobs, bench.num_machines);
                //         },
                //         criterion::BatchSize::SmallInput,
                //     )
                // });

                // Benchmark chunked algorithm with optimal chunk size
            }

            let (algo_name, algo_fn) = chunked_algorithm;
            group.bench_with_input(
                BenchmarkId::new(
                    algo_name,
                    format!(
                        "{}x{}_chunk{}",
                        bench.num_jobs, bench.num_machines, chunk_size
                    ),
                ),
                &(&flat_times, bench.num_jobs, bench.num_machines, chunk_size),
                |b, &(times, n_jobs, n_machines, chunk_size)| {
                    b.iter(|| {
                        let _order = algo_fn(
                            black_box(times),
                            black_box(n_jobs),
                            black_box(n_machines),
                            black_box(chunk_size),
                        );
                    })
                },
            );

            // Also benchmark chunked algorithm with different chunk sizes for comparison
            if bench.num_jobs >= 100 {
                // Only for larger problems
                for &chunk_multiplier in &[0.5, 1.5, 2.0] {
                    let alt_chunk_size = ((chunk_size as f64) * chunk_multiplier) as usize;
                    if alt_chunk_size > 0 && alt_chunk_size != chunk_size {
                        group.bench_with_input(
                            BenchmarkId::new(
                                format!("{}_alt", algo_name),
                                format!(
                                    "{}x{}_chunk{}",
                                    bench.num_jobs, bench.num_machines, alt_chunk_size
                                ),
                            ),
                            &(
                                &flat_times,
                                bench.num_jobs,
                                bench.num_machines,
                                alt_chunk_size,
                            ),
                            |b, &(times, n_jobs, n_machines, chunk_size)| {
                                b.iter(|| {
                                    let _order = algo_fn(
                                        black_box(times),
                                        black_box(n_jobs),
                                        black_box(n_machines),
                                        black_box(chunk_size),
                                    );
                                })
                            },
                        );
                    }
                }
            }

            group.finish();
        }
    }
}

criterion_group! {
    name=benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark_neh_versions
}
criterion_main!(benches);
