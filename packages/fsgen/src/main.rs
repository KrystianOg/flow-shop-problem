mod cli;
mod generate;

fn main() {
    cli::cli();
    // Example usage for generating flow-shop problem instance
    // let seed = 873654221; // example seed (e.g., ta001)
    // let num_jobs = 20;
    // let num_machines = 5;
    //
    // let processing_times = generate_flow_shop(seed, num_jobs, num_machines);
    //
    // // Print out the generated flow-shop matrix
    // println!("Generated Flow-Shop Problem:");
    // for (i, machine) in processing_times.iter().enumerate() {
    //     for (j, &time) in machine.iter().enumerate() {
    //         print!("Machine {} Job {}: {} ", i + 1, j + 1, time);
    //     }
    //     println!();
    // }
}
