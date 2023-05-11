use integration_dynamics::{
    methods::{Beeman, Euler, EulerMod, GearPredictorCorrector, IntegrationMethod},
    particle::Particle,
};

use crate::constants::{
    acceleration_function, DIM, INITIAL_ACCELERATION, INITIAL_POSITION, INITIAL_VELOCITY,
    PARTICLE_MASS,
};
use crate::{
    args::Integration,
    constants::{INITIAL_FIFTH_DERIVATIVE, INITIAL_FOURTH_DERIVATIVE, INITIAL_THIRD_DERIVATIVE},
};

pub struct Oscillator {
    pub particle: Particle<DIM>,
    pub integration_method: Box<dyn IntegrationMethod<DIM>>,
    pub delta_t: f64,
}

impl Oscillator {
    pub fn new(delta_t: f64, integration_method: Integration) -> Self {
        let mut particle: Particle<DIM> = Particle::new(
            INITIAL_POSITION,
            INITIAL_VELOCITY,
            INITIAL_ACCELERATION,
            0.0,
            PARTICLE_MASS,
        );

        let integration_method: Box<dyn IntegrationMethod<DIM>> = match integration_method {
            Integration::Euler => Box::new(Euler::new(acceleration_function)),
            Integration::EulerMod => Box::new(EulerMod::new(acceleration_function)),
            Integration::Beeman => Box::new(Beeman::new(
                acceleration_function,
                &mut [&mut particle],
                delta_t,
            )),
            Integration::GearPredictorCorrector => {
                let particles_to_init = vec![(
                    &mut particle,
                    vec![
                        INITIAL_THIRD_DERIVATIVE,
                        INITIAL_FOURTH_DERIVATIVE,
                        INITIAL_FIFTH_DERIVATIVE,
                    ],
                )];
                Box::new(GearPredictorCorrector::new(
                    acceleration_function,
                    true,
                    particles_to_init,
                ))
            }
        };

        Self {
            particle,
            integration_method,
            delta_t,
        }
    }

    pub fn run(&mut self, steps: usize) -> Vec<[f64; DIM]> {
        for _ in 0..steps {
            self.particle.set_derivatives(
                self.integration_method
                    .calculate_step(&self.particle, self.delta_t),
            );
        }

        self.particle.cloned_derivatives()
    }
}
