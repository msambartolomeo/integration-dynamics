use clap::Parser;
use integration_dynamics::Integration;

#[derive(Parser, Debug)]
#[command(name = "Oscillation Integration", author, version, about)]
pub struct Cli {
    #[arg(value_enum)]
    pub integration_method: Integration,

    #[arg(short, long, default_value_t = 1e-4)]
    pub simulation_delta_t: f64,

    #[arg(short, long, default_value_t = 1e-2)]
    pub output_delta_t: f64,

    #[arg(short, long, default_value_t = 5.0)]
    pub max_time: f64,

    #[arg(short, long, default_value_t = String::from("./oscillator.xyz"))]
    pub xyz_output_path: String,

    #[arg(short, long, default_value_t = String::from("./oscillator.txt"))]
    pub data_output_path: String,
}
