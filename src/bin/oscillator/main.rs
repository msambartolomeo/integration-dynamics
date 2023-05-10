use constants::{acceleration_function, DIM};
use integration_dynamics::methods::Euler;
use simulation::OscillatorSimulation;
use std::io::{BufWriter, Write};
use clap::Parser;

use args::{Cli, Integration};
mod args;
mod constants;
mod simulation;

fn main() {
    let integration_method: Euler<DIM> = Euler::new(acceleration_function);
    let args = Cli::parse();

    let mut simulation = OscillatorSimulation::new(0.0001, Box::new(integration_method));

    let steps = simulation.run(50000);

    // Write to file
    let file = std::fs::File::create("oscillator.xyz").unwrap();
    let mut writer = BufWriter::new(file);
    for step in steps {
        writeln!(writer, "3").unwrap();
        writeln!(writer, "Properties=pos:R:1:velo:R:1:radius:R:1",).unwrap();
        writeln!(writer, "{:.8} {:.8} 0.1", step[0][0], step[1][0]).unwrap();
        writeln!(writer, "-1.5, 0.0 0.0").unwrap();
        writeln!(writer, "1.5, 0.0 0.0").unwrap();
    }
}
