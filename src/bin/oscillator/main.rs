use anyhow::Result;
use clap::Parser;

use args::Cli;
use constants::analytic_solution;
use io::{output_data, output_simulation, Data};
use simulation::Oscillator;

mod args;
mod constants;
mod io;
mod simulation;

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut simulation = Oscillator::new(args.simulation_delta_t, args.integration_method);

    let output_iters = (args.max_time / args.output_delta_t) as usize;
    let simulation_iters = (args.output_delta_t / args.simulation_delta_t) as usize;

    let mut steps = Vec::new();
    let mut data = Vec::new();

    for i in 0..output_iters {
        let time = i as f64 * args.output_delta_t;

        let r = simulation.run(simulation_iters);

        let numeric_position = r[0][0];
        let analitic_position = analytic_solution(time);

        steps.push((r[0][0], r[1][0]));
        data.push(Data::new(time, numeric_position, analitic_position));
    }

    output_simulation(&args.xyz_output_path, &steps)?;
    output_data(&args.data_output_path, &data)?;

    Ok(())
}
