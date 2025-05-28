use std::fs::File;
use std::io::{self, Write};
use std::time::SystemTime;

fn unif(low: u32, high: u32, seed: &mut u32) -> u32 {
    // Parameters for the linear congruential generator
    const M: i32 = 2_147_483_647;
    const A: i32 = 16_807;
    const B: i32 = 127_773;
    const C: i32 = 2_836;

    // Convert seed to i32 for internal calculations
    let mut seed_i32 = *seed as i32;

    // Apply the Park-Miller LCG formula
    let k = seed_i32 / B;
    seed_i32 = A * (seed_i32 % B) - k * C;
    if seed_i32 < 0 {
        seed_i32 += M;
    }

    // Convert back to u32 and update the original seed
    *seed = seed_i32 as u32;

    // Calculate a floating-point value between 0 and 1
    let value_0_1 = seed_i32 as f64 / M as f64;

    low + (value_0_1 * ((high - low + 1) as f64)).trunc() as u32
}

pub fn generate_flow_shop(
    num_jobs: u16,
    num_machines: u16,
    seed: Option<u32>,
) -> (Vec<Vec<u32>>, u32) {
    let mut rng_seed = seed.unwrap_or(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .try_into()
            .unwrap(),
    );
    let mut processing_times: Vec<Vec<u32>> = Vec::with_capacity(num_machines as usize);

    for _ in 0..num_machines {
        let mut machine_times: Vec<u32> = Vec::with_capacity(num_jobs as usize);
        for _ in 0..num_jobs {
            machine_times.push(unif(1, 99, &mut rng_seed));
        }
        processing_times.push(machine_times);
    }

    (processing_times, rng_seed)
}

pub fn save_to_file(filename: &str, processing_times: &[Vec<u32>], seed: u32) -> io::Result<()> {
    let mut file = File::create(filename)?;
    let num_machines = processing_times.len();
    let num_jobs = processing_times[0].len();

    writeln!(
        file,
        "number of jobs, number of machines, initial seed, upper bound and lower bound :"
    )?;
    writeln!(file, "{num_jobs} {num_machines} {seed} ? ?")?;
    writeln!(file, "processing times :")?;

    // Writing the problem size: Number of jobs (columns) and machines (rows)
    writeln!(file, "{num_jobs} {num_machines}")?;

    // Writing the processing times: Machines (rows) and jobs (columns)
    for i in 0..num_machines {
        let machine_times: Vec<String> = processing_times[i]
            .iter()
            .map(|&time| time.to_string())
            .collect();
        writeln!(file, "{}", machine_times.join(" "))?;
    }

    Ok(())
}
