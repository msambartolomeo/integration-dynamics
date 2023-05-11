use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "Billiards Integration", author, version, about)]
pub struct Cli {
    #[arg(value_enum)]
    pub integration_method: Integration,

    #[arg(short, long, default_value_t = 1e-4)]
    pub simulation_delta_t: f64,

    #[arg(short, long, default_value_t = 1e-2)]
    pub output_delta_t: f64,

    #[arg(short, long, default_value_t = 5.0)]
    pub max_time: f64,

    #[arg(short, long, default_value_t = String::from("./billiards.xyz"))]
    pub xyz_output_path: String,

    #[arg(short, long, default_value_t = String::from("./billiards.txt"))]
    pub data_output_path: String,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Integration {
    Euler,
    EulerMod,
    Verlet,
    Beeman,
    GearPredictorCorrector,
}
