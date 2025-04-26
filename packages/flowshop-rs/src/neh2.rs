use std::ptr;

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
    let mut best_sequence = Vec::with_capacity(n_jobs);

    let mut completion = vec![0; n_machines];
    let mut temp_completion = vec![0; n_machines];

    sequence.push(jobs[0]);

    for &job in &jobs[1..] {
        let mut best_makespan = usize::MAX;
        best_sequence.clear();

        for pos in 0..=sequence.len() {
            best_sequence.extend_from_slice(&sequence[..pos]);
            best_sequence.push(job);
            best_sequence.extend_from_slice(&sequence[pos..]);

            let makespan = unsafe {
                calculate_makespan_unchecked(
                    times,
                    &best_sequence,
                    n_machines,
                    &mut temp_completion,
                )
            };

            if makespan < best_makespan {
                best_makespan = makespan;
                sequence.clear();
                sequence.extend_from_slice(&best_sequence);
            }

            best_sequence.clear();
        }
    }

    sequence
}

#[inline(always)]
unsafe fn calculate_makespan_unchecked(
    times: &[usize],
    sequence: &[usize],
    n_machines: usize,
    completion: &mut [usize],
) -> usize {
    let mut comp_ptr = completion.as_mut_ptr();
    for i in 0..n_machines {
        ptr::write(comp_ptr.add(i), 0);
    }

    for &job in sequence {
        let job_ptr = times.as_ptr().add(job * n_machines);

        let first = comp_ptr;
        let first_time = *job_ptr;
        ptr::write(first, *first + first_time);

        for m in 1..n_machines {
            let prev = *comp_ptr.add(m - 1);
            let current = *comp_ptr.add(m);
            let time = *job_ptr.add(m);
            ptr::write(comp_ptr.add(m), prev.max(current) + time);
        }
    }

    *comp_ptr.add(n_machines - 1)
}
