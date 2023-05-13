use std::fs::File;

use anyhow::{Ok, Result};
use args::Cli;
use clap::Parser;
use constants::INITIAL_WHITE_BALL_VELOCITY;
use io::output_simulation;
use simulation::Billiards;

mod args;
mod constants;
mod io;
mod simulation;

fn main() -> Result<()> {
    let args = Cli::parse();
    let file = File::create(args.xyz_output_path)?;

    let mut simulation = Billiards::new(
        args.simulation_delta_t,
        &args.integration_method,
        args.fixed_spacing,
        args.white_offset,
        INITIAL_WHITE_BALL_VELOCITY,
        !args.ignore_holes,
    );

    let output_iters = (args.max_time / args.output_delta_t) as usize;
    let simulation_iters = (args.output_delta_t / args.simulation_delta_t) as usize;

    output_simulation(&file, simulation.balls(), !args.ignore_holes)?;

    for _ in 1..=output_iters {
        let particles = simulation.run(simulation_iters);

        output_simulation(&file, particles, !args.ignore_holes)?;
    }

    Ok(())
}
