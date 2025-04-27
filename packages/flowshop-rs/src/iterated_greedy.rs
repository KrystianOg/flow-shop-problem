use crate::neh::{makespan, neh};
use rand::rng;
use rand::seq::SliceRandom;

type Order = Vec<usize>;

pub fn iterated_greedy(
    times: &[usize],
    n_jobs: usize,
    n_machines: usize,
    max_iterations: usize,
    destruction_size: usize,
) -> Order {
    let mut best_order = neh(times, n_jobs, n_machines);
    let mut best_makespan = makespan(&best_order, times, &mut vec![0; n_machines], n_machines);

    for _ in 0..max_iterations {
        // Destruction phase: randomly remove a subset of jobs
        let mut rng = rng();
        let mut remaining_jobs: Vec<usize> = (1..=n_jobs).collect();
        remaining_jobs.shuffle(&mut rng);
        let destroyed_jobs: Vec<usize> =
            remaining_jobs.into_iter().take(destruction_size).collect();

        // Construction phase: reconstruct the schedule with the remaining jobs
        let mut new_order = best_order.clone();
        for job in destroyed_jobs {
            new_order.retain(|&x| x != job);
        }
        let new_order = neh(times, n_jobs, n_machines); // Reconstruct using NEH

        // Evaluate the new schedule
        let new_makespan = makespan(&new_order, times, &mut vec![0; n_machines], n_machines);

        // Acceptance criterion
        if new_makespan < best_makespan {
            best_order = new_order;
            best_makespan = new_makespan;
        }
    }

    best_order
}
