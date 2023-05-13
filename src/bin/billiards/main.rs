use anyhow::{Ok, Result};
use args::Cli;
use clap::Parser;
use simulation::Billiards;

mod args;
mod constants;
mod simulation;

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut simulation = Billiards::new(
        args.simulation_delta_t,
        args.integration_method,
        args.fixed_spacing,
        args.white_offset,
        0.1,
    );

    let output_iters = (args.max_time / args.output_delta_t) as usize;
    let simulation_iters = (args.output_delta_t / args.simulation_delta_t) as usize;

    let mut steps = Vec::new();

    for i in 1..=output_iters {
        let r = simulation.run(simulation_iters);
        let numeric_position = r[0][0];

        steps.push((r[0][0], r[1][0]));
    }

    todo!();

    Ok(())
}
