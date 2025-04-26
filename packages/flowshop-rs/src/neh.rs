pub type Job = Vec<usize>;
pub type Order = Vec<usize>;

pub fn makespan(perm: &Order, times: &[Job], buffer: &mut [usize], machine_count: usize) -> usize {
    buffer.fill(0);

    for &job_id in perm.iter() {
        for machine in 0..machine_count {
            let left = if machine > 0 { buffer[machine - 1] } else { 0 };
            buffer[machine] = std::cmp::max(buffer[machine], left) + times[job_id - 1][machine];
        }
    }

    buffer[machine_count - 1]
}

pub fn neh(times: &[Job]) -> Order {
    let mut jobs_with_total_times: Vec<(usize, usize)> = times
        .iter()
        .enumerate()
        .map(|(job_id, job)| (job_id + 1, job.iter().sum()))
        .collect();

    jobs_with_total_times.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let machine_count = times[0].len();
    let mut buffer = vec![0; machine_count];

    // pre-allocate `order`
    let mut temp_order: Order = Vec::with_capacity(times.len());

    let mut final_order: Order = Vec::with_capacity(times.len());

    for &(job_id, _) in &jobs_with_total_times {
        let mut best_makespan = usize::MAX;
        let mut best_position = 0;

        // save original order
        for pos in 0..=final_order.len() {
            temp_order.clear();
            temp_order.extend_from_slice(&final_order[..pos]);
            temp_order.push(job_id);
            temp_order.extend_from_slice(&final_order[pos..]);

            let current_makespan = makespan(&temp_order, times, &mut buffer, machine_count);
            if current_makespan < best_makespan {
                best_makespan = current_makespan;
                best_position = pos;
            }
        }

        final_order.insert(best_position, job_id);
    }

    final_order
}

pub fn calculate_cmax(optimal_order: &[usize], times: &[Job]) -> usize {
    let machine_count = times[0].len();

    let mut completion_times = vec![0; machine_count];

    for &job_id in optimal_order {
        let mut prev = 0;
        for (machine, buf) in completion_times.iter_mut().enumerate() {
            prev = prev.max(*buf) + times[job_id - 1][machine];
            *buf = prev;
        }
    }

    completion_times[machine_count - 1]
}
