pub fn nehh(times: &[usize], n_jobs: usize, n_machines: usize) -> Vec<usize> {
    let mut jobs: Vec<usize> = (0..n_jobs).collect();
    jobs.sort_unstable_by_key(|&i| {
        std::cmp::Reverse(
            times[i * n_machines..(i + 1) * n_machines]
                .iter()
                .sum::<usize>(),
        )
    });

    let mut sequence = Vec::with_capacity(n_jobs);
    let mut temp_sequence = Vec::with_capacity(n_jobs);

    let mut completion = vec![0; n_machines];
    let mut temp_completion = vec![0; n_machines];

    sequence.push(jobs[0]);

    for &job in &jobs[1..] {
        let mut best_makespan = usize::MAX;
        let mut best_position = 0;

        for pos in 0..=sequence.len() {
            temp_sequence.clear();
            temp_sequence.extend_from_slice(&sequence[..pos]);
            temp_sequence.push(job);
            temp_sequence.extend_from_slice(&sequence[pos..]);

            let makespan =
                calculate_makespan_fast(times, &temp_sequence, n_machines, &mut temp_completion);

            if makespan < best_makespan {
                best_makespan = makespan;
                best_position = pos;
            }
        }

        sequence.insert(best_position, job);
    }

    sequence
}

fn calculate_makespan_fast(
    times: &[usize],
    sequence: &[usize],
    n_machines: usize,
    completion: &mut [usize],
) -> usize {
    completion.fill(0);

    for &job in sequence {
        completion[0] += times[job * n_machines];
        for m in 1..n_machines {
            completion[m] = completion[m].max(completion[m - 1]) + times[job * n_machines + m];
        }
    }

    completion[n_machines - 1]
}
