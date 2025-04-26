#![feature(portable_simd)]

use std::ptr;
use std::simd::cmp::SimdOrd;
use std::simd::Simd;

const LANES: usize = 8;
type UsizexN = Simd<usize, LANES>;

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
    let mut completion = vec![0; n_machines];

    sequence.push(jobs[0]);

    for &job in &jobs[1..] {
        let mut best_makespan = usize::MAX;
        let mut best_position = 0;

        for pos in 0..=sequence.len() {
            let makespan = unsafe {
                simulate_insert_and_calculate(
                    times,
                    &sequence,
                    job,
                    pos,
                    n_machines,
                    &mut completion,
                )
            };

            if makespan < best_makespan {
                best_makespan = makespan;
                best_position = pos;
            }
        }

        sequence.insert(best_position, job);
    }

    sequence
}

#[inline(always)]
unsafe fn simulate_insert_and_calculate(
    times: &[usize],
    sequence: &[usize],
    job: usize,
    insert_pos: usize,
    n_machines: usize,
    completion: &mut [usize],
) -> usize {
    let mut comp_ptr = completion.as_mut_ptr();

    for i in 0..n_machines {
        ptr::write(comp_ptr.add(i), 0);
    }

    let mut idx = 0;
    let total_jobs = sequence.len() + 1;

    for _ in 0..total_jobs {
        let job_idx = if idx == insert_pos {
            job
        } else {
            let real_idx = if idx < insert_pos { idx } else { idx - 1 };
            sequence[real_idx]
        };

        let job_ptr = times.as_ptr().add(job_idx * n_machines);

        // Process first machine separately
        let first = comp_ptr;
        let first_time = *job_ptr;
        ptr::write(first, *first + first_time);

        let mut m = 1;

        // SIMD block: process machines in chunks of LANES
        while m + LANES <= n_machines {
            let comp_slice = std::slice::from_raw_parts_mut(comp_ptr.add(m), LANES);
            let mut comp_vec = UsizexN::from_slice(comp_slice);

            let prev_slice = std::slice::from_raw_parts(comp_ptr.add(m - 1), LANES);
            let prev_vec = UsizexN::from_slice(prev_slice);

            let times_slice = std::slice::from_raw_parts(job_ptr.add(m), LANES);
            let times_vec = UsizexN::from_slice(times_slice);

            comp_vec = comp_vec.simd_max(prev_vec) + times_vec;
            comp_vec.copy_to_slice(comp_slice);

            m += LANES;
        }

        // Scalar fallback for remainder
        for mm in m..n_machines {
            let prev = *comp_ptr.add(mm - 1);
            let current = *comp_ptr.add(mm);
            let time = *job_ptr.add(mm);
            ptr::write(comp_ptr.add(mm), prev.max(current) + time);
        }

        idx += 1;
    }

    *comp_ptr.add(n_machines - 1)
}
