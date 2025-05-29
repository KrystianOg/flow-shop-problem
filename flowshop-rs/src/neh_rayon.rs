use crate::neh::makespan;
use rayon::prelude::*;

type Order = Vec<usize>;

pub fn neh(times: &[usize], n_jobs: usize, n_machines: usize) -> Vec<usize> {
    let mut jobs_with_totals: Vec<(usize, usize)> = (0..n_jobs)
        .into_par_iter()
        .map(|job_id| {
            let start = job_id * n_machines;
            let end = start + n_machines;
            (job_id + 1, times[start..end].iter().sum())
        })
        .collect();

    jobs_with_totals.sort_unstable_by_key(|&(_, total)| std::cmp::Reverse(total));

    let mut final_order: Order = Vec::with_capacity(n_jobs);

    for &(job_id, _) in &jobs_with_totals {
        let best_result = (0..=final_order.len())
            .into_par_iter()
            .map(|pos| {
                let mut temp_order = Vec::with_capacity(final_order.len() + 1);
                temp_order.extend_from_slice(&final_order[..pos]);
                temp_order.push(job_id);
                temp_order.extend_from_slice(&final_order[pos..]);

                let mut local_buffer = vec![0; n_machines];
                let makespan_value = makespan(&temp_order, times, &mut local_buffer, n_machines);

                (makespan_value, pos)
            })
            .min_by_key(|&(makespan_value, _)| makespan_value)
            .expect("At least one postion should exist");

        final_order.insert(best_result.1, job_id);
    }

    final_order
}

// Alternative implementation with better memory efficiency for larger problems
pub fn neh_parallel_optimized(times: &[usize], n_jobs: usize, n_machines: usize) -> Vec<usize> {
    let mut jobs_with_totals: Vec<(usize, usize)> = (0..n_jobs)
        .map(|job_id| {
            let start = job_id * n_machines;
            let total_time = times[start..start + n_machines].iter().sum();
            (job_id + 1, total_time)
        })
        .collect();

    jobs_with_totals.sort_unstable_by_key(|&(_, total)| std::cmp::Reverse(total));

    let mut final_order = Vec::with_capacity(n_jobs);

    for &(job_id, _) in &jobs_with_totals {
        let current_len = final_order.len();

        // Use parallel iterator with reduce for better performance on large datasets
        let (best_makespan, best_pos) = (0..=current_len)
            .into_par_iter()
            .map(|pos| {
                // More efficient temporary order construction
                let mut temp_order = Vec::with_capacity(current_len + 1);

                // Use extend_from_slice for better performance
                if pos > 0 {
                    temp_order.extend_from_slice(&final_order[..pos]);
                }
                temp_order.push(job_id);
                if pos < current_len {
                    temp_order.extend_from_slice(&final_order[pos..]);
                }

                let mut local_buffer = vec![0; n_machines];
                let makespan_value = makespan(&temp_order, times, &mut local_buffer, n_machines);

                (makespan_value, pos)
            })
            .reduce_with(|acc, curr| if curr.0 < acc.0 { curr } else { acc })
            .expect("At least one position should exist");

        final_order.insert(best_pos, job_id);
    }

    final_order
}

// For very large problems, consider this chunked approach to reduce memory pressure
pub fn neh_parallel_chunked(
    times: &[usize],
    n_jobs: usize,
    n_machines: usize,
    chunk_size: usize,
) -> Vec<usize> {
    let mut jobs_with_totals: Vec<(usize, usize)> = (0..n_jobs)
        .map(|job_id| {
            let start = job_id * n_machines;
            let total_time = times[start..start + n_machines].iter().sum();
            (job_id + 1, total_time)
        })
        .collect();

    jobs_with_totals.sort_unstable_by_key(|&(_, total)| std::cmp::Reverse(total));

    let mut final_order = Vec::with_capacity(n_jobs);

    for &(job_id, _) in &jobs_with_totals {
        let current_len = final_order.len();

        // Process positions in chunks to balance parallelism with memory usage
        let best_result = (0..=current_len)
            .collect::<Vec<_>>()
            .par_chunks(chunk_size.max(1))
            .flat_map(|chunk| {
                chunk.par_iter().map(|&pos| {
                    let mut temp_order = Vec::with_capacity(current_len + 1);
                    temp_order.extend_from_slice(&final_order[..pos]);
                    temp_order.push(job_id);
                    temp_order.extend_from_slice(&final_order[pos..]);

                    let mut local_buffer = vec![0; n_machines];
                    let makespan_value =
                        makespan(&temp_order, times, &mut local_buffer, n_machines);

                    (makespan_value, pos)
                })
            })
            .min_by_key(|&(makespan_value, _)| makespan_value)
            .expect("At least one position should exist");

        final_order.insert(best_result.1, job_id);
    }

    final_order
}
