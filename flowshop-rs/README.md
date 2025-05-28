# Flow shop scheduling problem - Rust

This directory consists of different heuristics and meta-heuristics for solving Flow Shop Scheduling Problem (FSP).

## High level overview

lib.rs exports different modules to a single `flowshop-rs` package for further reference.

TODO: main.rs runs cli benchmarks generator. (currently this feature is available in different package `fsp-gen`).

iterated_greedy.rs - as name suggest iterated greed + neh
neh.rs - includes NEH heuristic implementation.
neh_rayon.rs - consists of similar NEH heuristic but multithreaded.
reader.rs - reader for taillard benchmark files

## CLI

Cli requirements:

- it can generate instances according to Taillard generation algorithm

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

### About Taillard

### About VRF

https://zenodo.org/records/3550553

https://github.com/afkummer/ufrgs-inf05010-2019-1
https://github.com/afkummer/ufrgs-inf05010-2019-1/tree/master/instances/pfsp
https://gitlab.univ-lille.fr/ultrabo/pbb
https://gitlab.univ-lille.fr/ultrabo/pbb/-/tree/master/evaluation/flowshop/data/vrf_parameters

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
