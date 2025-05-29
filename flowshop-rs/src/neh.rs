use std::collections::HashMap;

type Order = Vec<usize>;

#[inline(always)]
pub fn makespan(perm: &[usize], times: &[usize], buffer: &mut [usize], n_machines: usize) -> usize {
    buffer.fill(0);

    for &job_id in perm {
        let job_start = (job_id - 1) * n_machines;
        let job_times = &times[job_start..job_start + n_machines];

        let mut prev = 0;
        for (machine_time, completion_time) in job_times.iter().zip(buffer.iter_mut()) {
            prev = prev.max(*completion_time) + machine_time;
            *completion_time = prev;
        }
    }

    buffer[n_machines - 1]
}

fn cached_makespan(
    perm: &[usize],
    times: &[usize],
    buffer: &mut [usize],
    n_machines: usize,
    cache: &mut HashMap<Vec<usize>, usize>,
) -> usize {
    if let Some(&cached_result) = cache.get(perm) {
        return cached_result;
    }

    let result = makespan(perm, times, buffer, n_machines);
    cache.insert(perm.to_vec(), result);
    result
}

pub fn neh(times: &[usize], n_jobs: usize, n_machines: usize) -> Vec<usize> {
    // for each job, calculate the sum of its processing times across all machines
    let mut jobs_with_totals: Vec<(usize, usize)> = (0..n_jobs)
        .map(|job_id| {
            let start = job_id * n_machines;
            let total_time = times[start..start + n_machines].iter().sum();
            (job_id + 1, total_time)
        })
        .collect();

    // sort jobs by descending total time
    jobs_with_totals.sort_unstable_by_key(|&(_, total)| std::cmp::Reverse(total));

    let mut buffer = vec![0; n_machines];
    let mut temp_order: Order = Vec::with_capacity(n_jobs);
    let mut final_order: Order = Vec::with_capacity(n_jobs + 1);

    // build sequence
    for &(job_id, _) in &jobs_with_totals {
        let mut best_makespan = usize::MAX;
        let mut best_position = 0;

        // try inserting at every possible position
        for pos in 0..=final_order.len() {
            temp_order.clear();
            temp_order.extend_from_slice(&final_order[..pos]);
            temp_order.push(job_id);
            temp_order.extend_from_slice(&final_order[pos..]);

            // calculate makesapn for each possible insertion
            let current_makespan = makespan(&temp_order, times, &mut buffer, n_machines);
            if current_makespan < best_makespan {
                best_makespan = current_makespan;
                best_position = pos;
            }
        }

        final_order.insert(best_position, job_id);
    }

    final_order
}

pub fn calculate_cmax(optimal_order: &[usize], times: &[usize], n_machines: usize) -> usize {
    let mut completion_times = vec![0; n_machines];

    for &job_id in optimal_order {
        let job_start = (job_id - 1) * n_machines;
        let job_times = &times[job_start..job_start + n_machines];

        let mut prev = 0;
        for (machine_time, completion_time) in job_times.iter().zip(completion_times.iter_mut()) {
            prev = prev.max(*completion_time) + machine_time;
            *completion_time = prev;
        }
    }

    completion_times[n_machines - 1]
}
