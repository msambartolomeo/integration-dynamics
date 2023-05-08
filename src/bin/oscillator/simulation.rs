use integration_dynamics::{methods::IntegrationMethod, particle::Particle};

use crate::constants::{
    DIM, INITIAL_ACCELERATION, INITIAL_POSITION, INITIAL_VELOCITY, PARTICLE_MASS,
};

pub struct OscillatorSimulation {
    pub particle: Particle<DIM>,
    pub integration_method: Box<dyn IntegrationMethod<DIM>>,
    pub delta_t: f64,
}

impl OscillatorSimulation {
    pub fn new(delta_t: f64, integration_method: Box<dyn IntegrationMethod<DIM>>) -> Self {
        let particle: Particle<DIM> = Particle::new(
            INITIAL_POSITION,
            INITIAL_VELOCITY,
            INITIAL_ACCELERATION,
            0.0,
            PARTICLE_MASS,
        );

        Self {
            particle,
            integration_method,
            delta_t,
        }
    }

    pub fn run(&mut self, steps: usize) -> Vec<Vec<[f64; DIM]>> {
        let mut positions = vec![];
        positions.push(self.particle.cloned_derivatives());
        for _ in 0..steps {
            self.particle.set_derivatives(
                self.integration_method
                    .calculate_step(&self.particle, self.delta_t),
            );
            positions.push(self.particle.cloned_derivatives());
        }
        positions
    }
}
