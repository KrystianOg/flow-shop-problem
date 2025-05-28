use std::collections::HashMap;

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

pub fn cached_makespan(
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
    let mut jobs_with_total_times: Vec<(usize, usize)> = (0..n_jobs)
        .map(|job_id| {
            let start = job_id * n_machines;
            let end = start + n_machines;
            (job_id + 1, times[start..end].iter().sum())
        })
        .collect();

    jobs_with_total_times.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let mut buffer = vec![0; n_machines];

    // pre-allocate `order`
    let mut temp_order: Order = Vec::with_capacity(n_jobs);

    let mut final_order: Order = Vec::with_capacity(n_jobs);

    for &(job_id, _) in &jobs_with_total_times {
        let mut best_makespan = usize::MAX;
        let mut best_position = 0;

        // save original order
        for pos in 0..=final_order.len() {
            temp_order.clear();
            temp_order.extend_from_slice(&final_order[..pos]);
            temp_order.push(job_id);
            temp_order.extend_from_slice(&final_order[pos..]);

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
        let mut prev = 0;
        for (machine, buf) in completion_times.iter_mut().enumerate() {
            prev = prev.max(*buf) + times[job_id + machine];
            *buf = prev;
        }
    }

    completion_times[n_machines - 1]
}
