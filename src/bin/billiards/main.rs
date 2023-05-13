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

    let include_holes = args.include_holes;

    let mut simulation = Billiards::new(
        args.simulation_delta_t,
        &args.integration_method,
        args.fixed_spacing,
        args.white_offset,
        INITIAL_WHITE_BALL_VELOCITY,
    );

    let output_iters = (args.max_time / args.output_delta_t) as usize;
    let simulation_iters = (args.output_delta_t / args.simulation_delta_t) as usize;

    output_simulation(&file, simulation.balls(), include_holes)?;

    for _ in 1..=output_iters {
        let particles = simulation.run(simulation_iters);

        output_simulation(&file, particles, include_holes)?;
    }

    Ok(())
}
