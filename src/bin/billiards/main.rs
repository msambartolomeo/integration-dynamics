use std::fs::File;

use anyhow::{Ok, Result};
use clap::Parser;

use args::Cli;
use constants::INITIAL_WHITE_BALL_VELOCITY;
use io::{output_positions, output_simulation};
use simulation::Billiards;

mod args;
mod constants;
mod io;
mod simulation;

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut data_file = None;
    if let Some(path) = args.data_output_path {
        data_file = Some(File::create(path)?);
    }

    let xyz_file = File::create(args.xyz_output_path)?;

    let mut simulation = Billiards::new(
        args.simulation_delta_t,
        &args.integration_method,
        args.fixed_spacing,
        args.white_offset,
        INITIAL_WHITE_BALL_VELOCITY,
        !args.ignore_holes,
        args.ball_count_stop_condition,
    );

    let output_iters = (args.max_time / args.output_delta_t) as usize;
    let simulation_iters = (args.output_delta_t / args.simulation_delta_t) as usize;

    if let Some(file) = &data_file {
        output_positions(file, simulation.balls(), 0.0)?;
    }
    output_simulation(&xyz_file, simulation.balls(), !args.ignore_holes)?;

    for i in 1..=output_iters {
        let particles = simulation.run(simulation_iters);

        output_simulation(&xyz_file, particles, !args.ignore_holes)?;

        if let Some(file) = &data_file {
            let time = i as f64 * args.output_delta_t;

            output_positions(file, particles, time)?;
        }
        if particles.len() == args.ball_count_stop_condition {
            break;
        }
    }

    Ok(())
}
