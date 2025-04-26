use std::fs::File;
use std::io::{self, Write};

fn unif(seed: &mut u32, low: u32, high: u32) -> u32 {
    // Parameters for the linear congruential generator
    const M: u32 = 2147483647;
    const A: u32 = 16807;
    const B: u32 = 127773;
    const C: u32 = 2836;

    let k = *seed / B;
    *seed = A * (*seed % B) - k * C;
    // *seed = seed.wrapping_add(M);

    let value_0_1 = *seed as f64 / M as f64;

    let v = low + (value_0_1 * (high - low + 1) as f64).floor() as u32;
    println!("value {value_0_1}");

    v
}

pub fn generate_flow_shop(seed: u32, num_jobs: u16, num_machines: u16) -> Vec<Vec<u32>> {
    let mut rng_seed = seed;
    let mut processing_times: Vec<Vec<u32>> = Vec::with_capacity(num_machines as usize);

    for _ in 0..num_jobs {
        let mut machine_times: Vec<u32> = Vec::with_capacity(num_jobs as usize);
        for _ in 0..num_machines {
            machine_times.push(unif(&mut rng_seed, 1, 99));
        }
        processing_times.push(machine_times);
    }

    processing_times
}

pub fn save_to_file(filename: &str, processing_times: &[Vec<u32>]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    let num_machines = processing_times.len();
    let num_jobs = processing_times[0].len();

    // Writing the problem size
    writeln!(file, "{num_jobs} {num_machines}")?;

    // Writing the processing times
    for i in 0..num_jobs {
        let job_times: Vec<String> = processing_times
            .iter()
            .map(|machine| machine[i].to_string())
            .collect();
        writeln!(file, "{}", job_times.join(" "))?;
    }

    Ok(())
}
