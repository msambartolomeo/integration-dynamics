use integration_dynamics::{methods::IntegrationMethod, particle::Particle};

use crate::constants::{
    acceleration_function, DIM, INITIAL_POSITION, INITIAL_VELOCITY, PARTICLE_MASS,
};

pub struct OscillatorSimulation {
    pub particle: Particle<DIM>,
    pub integration_method: Box<dyn IntegrationMethod<DIM>>,
    pub delta_t: f64,
    pub acceleration_function: fn(&Particle<DIM>) -> f64,
}

impl OscillatorSimulation {
    pub fn new(delta_t: f64, integration_method: Box<dyn IntegrationMethod<DIM>>) -> Self {
        let particle: Particle<DIM> =
            Particle::new(INITIAL_POSITION, INITIAL_VELOCITY, 0.0, PARTICLE_MASS);

        Self {
            particle,
            integration_method,
            delta_t,
            acceleration_function,
        }
    }

    pub fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            todo!();
        }
    }
}
