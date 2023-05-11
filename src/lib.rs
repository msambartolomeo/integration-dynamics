use clap::ValueEnum;

pub mod methods;
pub mod particle;

#[derive(ValueEnum, Clone, Debug)]
pub enum Integration {
    Euler,
    EulerMod,
    Verlet,
    Beeman,
    GearPredictorCorrector,
}
