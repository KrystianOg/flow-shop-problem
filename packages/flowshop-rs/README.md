# Flow shop scheduling problem - Rust

This directory consists of different heuristics and meta-heuristics for solving Flow Shop Scheduling Problem (FSP).

## High level overview

lib.rs exports different modules to a single `flowshop-rs` package for further reference.

TODO: main.rs runs cli benchmarks generator. (currently this feature is available in different package `fsgen`).

terated_greedy - as name suggest iterated greed + neh
neh.rs - includes NEH heuristic implementation.
neh_rayon.rs - consists of similar NEH heuristic but multithreaded.
reader.rs - reader for taillard benchmark files

TODO: solver.rs - runs our solver implementation.

## Setup

install deps:
```bash
cargo install
```

compile: 
```bash
cargo build --release
```

## benchmarking 

For ease of use I've included `criterion` package, which allows benchmarking functions in batches comparing different versions between runs in terms of performance.
Benchmarks are written in `benches/` directory typical benchmark consists of datasets we want to run, and optionally different algo versions e.g. `neh` vs `neh_rayon` (but I'll probably augment this as different bench).

```bash
cargo bench --bench <benchmark_name>
```

## Optimizing

run:

```bash
cargo flamegraph --bench benchmark_<name> -- --bench
```

## Running

run the `flamegraph` in rust

You probably have to set

```bash
sudo sysctl kernel.perf_event_paranoid=1
```

temporarily.
