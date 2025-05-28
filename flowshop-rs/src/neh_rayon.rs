use rayon::prelude::*;
// use std::sync::{Arc, Mutex};

type Order = Vec<usize>;

#[inline(always)]
pub fn makespan(perm: &[usize], times: &[usize], buffer: &mut [usize], n_machines: usize) -> usize {
    buffer.fill(0);

    for &job_id in perm {
        let mut buf_ptr = buffer.as_mut_ptr();

        unsafe {
            let mut times_ptr = times.as_ptr().add((job_id - 1) * n_machines);
            let mut prev = 0;
            let mut remaining = n_machines;

            while remaining != 0 {
                let up = *buf_ptr;
                let left = prev;
                let value = if up > left { up } else { left } + *times_ptr;

                *buf_ptr = value;
                prev = value;

                buf_ptr = buf_ptr.add(1);
                times_ptr = times_ptr.add(1);
                remaining -= 1;
            }
        }
    }

    buffer[n_machines - 1]
}

pub fn neh(times: &[usize], n_jobs: usize, n_machines: usize) -> Vec<usize> {
    // Parallelize the calculation of the total times for each job
    let jobs_with_total_times: Vec<(usize, usize)> = (0..n_jobs)
        .into_par_iter() // Parallel iterator
        .map(|job_id| {
            let start = job_id * n_machines;
            let end = start + n_machines;
            (job_id + 1, times[start..end].iter().sum())
        })
        .collect();

    // Sort jobs based on their total times (still a sequential operation)
    let mut jobs_with_total_times = jobs_with_total_times;
    jobs_with_total_times.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    // let mut buffer = vec![0; n_machines];
    // let buffer = Arc::new(Mutex::new(vec![0; n_machines]));
    // let mut temp_order: Order = Vec::with_capacity(n_jobs);
    let mut final_order: Order = Vec::with_capacity(n_jobs);

    for &(job_id, _) in &jobs_with_total_times {
        let mut best_makespan = usize::MAX;
        let mut best_position = 0;

        // We perform the search for the best position sequentially
        let results: Vec<(usize, usize)> = (0..=final_order.len())
            .into_par_iter() // Parallel iterator for positions
            .map(|pos| {
                // let mut buffer = buffer.lock().unwrap();
                let mut temp_order = final_order.clone();
                temp_order.insert(pos, job_id);

                let mut local_buffer = vec![0; n_machines];
                let current_makespan = makespan(&temp_order, times, &mut local_buffer, n_machines);
                (current_makespan, pos)
            })
            .collect();

        // Find the best position with the minimum makespan
        for (current_makespan, pos) in results {
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
        let mut prev = 0;
        for (machine, buf) in completion_times.iter_mut().enumerate() {
            prev = prev.max(*buf) + times[job_id + machine];
            *buf = prev;
        }
    }

    completion_times[n_machines - 1]
}
