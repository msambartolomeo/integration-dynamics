use clap::Parser;
use integration_dynamics::Integration;

#[derive(Parser, Debug)]
#[command(name = "Billiards Integration", author, version, about)]
pub struct Cli {
    #[arg(short, long, default_value_t = 0)]
    pub ball_count_stop_condition: usize,

    #[arg(short, long)]
    pub ignore_holes: bool,

    #[arg(value_enum)]
    pub integration_method: Integration,

    #[arg(short, long, default_value_t = false)]
    pub fixed_spacing: bool,

    #[arg(short, long, default_value_t = 0.0)]
    pub white_offset: f64,

    #[arg(short, long, default_value_t = 1e-4)]
    pub simulation_delta_t: f64,

    #[arg(short, long, default_value_t = 5e-2)]
    pub output_delta_t: f64,

    #[arg(short, long, default_value_t = 100.0)]
    pub max_time: f64,

    #[arg(short, long, default_value_t = String::from("./billiards.xyz"))]
    pub xyz_output_path: String,

    #[arg(short, long)]
    pub data_output_path: Option<String>,
}
