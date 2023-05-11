use integration_dynamics::{
    methods::{Beeman, Euler, EulerMod, GearPredictorCorrector, IntegrationMethod, Verlet},
    particle::Particle,
    Integration,
};

use crate::constants::{
    acceleration_function, DIM, INITIAL_ACCELERATION, INITIAL_FIFTH_DERIVATIVE,
    INITIAL_FOURTH_DERIVATIVE, INITIAL_POSITION, INITIAL_THIRD_DERIVATIVE, INITIAL_VELOCITY,
    PARTICLE_MASS,
};

pub struct Oscillator {
    particle: Particle<DIM>,
    integration_method: Box<dyn IntegrationMethod<DIM>>,
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
            Integration::Euler => Box::new(Euler::new(acceleration_function, delta_t)),
            Integration::EulerMod => Box::new(EulerMod::new(acceleration_function, delta_t)),
            Integration::Verlet => Box::new(Verlet::new(
                acceleration_function,
                &mut [&mut particle],
                delta_t,
            )),
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
                    delta_t,
                ))
            }
        };

        Self {
            particle,
            integration_method,
        }
    }

    pub fn run(&mut self, steps: usize) -> &[[f64; DIM]] {
        for _ in 0..steps {
            self.integration_method.advance_step(&mut self.particle);
        }

        self.particle.derivatives()
    }
}
