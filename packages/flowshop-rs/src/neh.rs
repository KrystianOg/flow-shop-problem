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

    jobs_with_total_times.sort_by(|a, b| b.1.cmp(&a.1));

    let machine_count = times[0].len();
    let mut buffer = vec![0; machine_count];

    // pre-allocate `order`
    let mut order: Order = Vec::with_capacity(times.len());
    // let mut order: Order = Vec::new();

    for &(job_id, _) in &jobs_with_total_times {
        let mut best_makespan = usize::MAX;
        let mut best_position = 0;

        // save original order
        order.push(job_id);
        for i in (0..order.len()).rev() {
            order.swap(i, i.saturating_sub(1));
            let current_makespan = makespan(&order, times, &mut buffer, machine_count);
            if current_makespan < best_makespan {
                best_makespan = current_makespan;
                best_position = i.saturating_sub(1);
            }

            // let mut temp_order = order.clone();
            // temp_order.insert(i, job_id);
            // let current_makespan = makespan(&temp_order, times);
            //
            // if current_makespan < best_makespan {
            //     best_makespan = current_makespan;
            //     best_position = i;
            // }
        }

        let inserted = order.pop().unwrap();
        order.insert(best_position, inserted);
    }

    order
}

pub fn calculate_cmax(optimal_order: &[usize], times: &[Job]) -> usize {
    let job_count = optimal_order.len();
    let machine_count = if !times.is_empty() { times[0].len() } else { 0 };

    let mut completion_times = vec![0; machine_count];

    for i in 0..job_count {
        let job_id = optimal_order[i] - 1;

        for machine in 0..machine_count {
            if job_id >= times.len() {
                continue;
            }
            if machine == 0 {
                completion_times[machine] =
                    completion_times[machine].max(0) + times[job_id][machine];
            } else {
                completion_times[machine] = completion_times[machine]
                    .max(completion_times[machine - 1])
                    + times[job_id][machine];
            }
        }
    }

    completion_times[machine_count - 1]
}
