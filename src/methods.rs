use crate::particle::Particle;

pub trait IntegrationMethod<const DIM: usize> {
    fn calculate_step(&self, particle: &Particle<DIM>, delta_t: f64) -> Vec<[f64; DIM]>;
}

pub struct Euler<const DIM: usize> {
    acceleration_function: fn(r: &[f64; DIM], v: &[f64; DIM], mass: f64) -> [f64; DIM],
}

impl<const DIM: usize> Euler<DIM> {
    pub fn new(
        acceleration_function: fn(r: &[f64; DIM], v: &[f64; DIM], mass: f64) -> [f64; DIM],
    ) -> Self {
        Self {
            acceleration_function,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for Euler<DIM> {
    fn calculate_step(&self, particle: &Particle<DIM>, delta_t: f64) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let mut new_r = particle.cloned_derivatives();

        for i in 0..DIM {
            new_r[0][i] += delta_t * r[1][i] + delta_t.powi(2) / 2.0 * r[2][i];
            new_r[1][i] += delta_t * r[2][i];
        }
        new_r[2] = (self.acceleration_function)(&new_r[0], &new_r[1], particle.mass());

        new_r
    }
}

pub struct EulerMod<const DIM: usize> {
    acceleration_function: fn(r: &[f64; DIM], v: &[f64; DIM], mass: f64) -> [f64; DIM],
}

impl<const DIM: usize> EulerMod<DIM> {
    pub fn new(
        acceleration_function: fn(r: &[f64; DIM], v: &[f64; DIM], mass: f64) -> [f64; DIM],
    ) -> Self {
        Self {
            acceleration_function,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for EulerMod<DIM> {
    fn calculate_step(&self, particle: &Particle<DIM>, delta_t: f64) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let mut new_r = particle.cloned_derivatives();

        for i in 0..DIM {
            new_r[1][i] += delta_t * r[2][i];
            new_r[0][i] += delta_t * new_r[1][i] + delta_t.powi(2) / 2.0 * r[2][i];
        }

        new_r[2] = (self.acceleration_function)(&new_r[0], &new_r[1], particle.mass());

        new_r
    }
}

pub struct Verlet<const DIM: usize> {
    acceleration_function: fn(r: &[f64; DIM], v: &[f64; DIM], mass: f64) -> [f64; DIM],
}

impl<const DIM: usize> Verlet<DIM> {
    pub fn new(
        acceleration_function: fn(r: &[f64; DIM], v: &[f64; DIM], mass: f64) -> [f64; DIM],
        particles_to_init: &mut [&mut Particle<DIM>],
        delta_t: f64,
    ) -> Self {
        let euler = Euler::new(acceleration_function);

        for particle in particles_to_init {
            let prev_derivatives = euler.calculate_step(&particle, -delta_t);
            particle.set_prev_derivatives(prev_derivatives);
        }

        Self {
            acceleration_function,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for Verlet<DIM> {
    fn calculate_step(&self, particle: &Particle<DIM>, delta_t: f64) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let old_r = particle.prev_derivatives();
        let mut new_r = particle.cloned_derivatives();

        for i in 0..DIM {
            new_r[0][i] *= 2.0;
            new_r[0][i] += -old_r[0][i] + delta_t.powi(2) * r[2][i];

            new_r[1][i] = (new_r[0][i] - old_r[0][i]) / (2.0 * delta_t);
        }

        new_r[2] = (self.acceleration_function)(&new_r[0], &new_r[1], particle.mass());

        new_r
    }
}

pub struct Beeman<const DIM: usize> {
    acceleration_function: fn(r: &[f64; DIM], v: &[f64; DIM], mass: f64) -> [f64; DIM],
}

impl<const DIM: usize> Beeman<DIM> {
    pub fn new(
        acceleration_function: fn(r: &[f64; DIM], v: &[f64; DIM], mass: f64) -> [f64; DIM],
        particles_to_init: &mut [&mut Particle<DIM>],
        delta_t: f64,
    ) -> Self {
        let euler = Euler::new(acceleration_function);

        for particle in particles_to_init {
            let prev_derivatives = euler.calculate_step(&particle, -delta_t);
            particle.set_prev_derivatives(prev_derivatives);
        }

        Self {
            acceleration_function,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for Beeman<DIM> {
    fn calculate_step(&self, particle: &Particle<DIM>, delta_t: f64) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let old_r = particle.prev_derivatives();
        let mut new_r = particle.cloned_derivatives();

        for i in 0..DIM {
            new_r[0][i] += r[1][i] * delta_t + 2.0 / 3.0 * r[2][i] * delta_t.powi(2)
                - 1.0 / 6.0 * old_r[2][i] * delta_t.powi(2);

            new_r[1][i] += 1.0 / 3.0 * new_r[2][i] * delta_t + 5.0 / 6.0 * r[2][i] * delta_t
                - 1.0 / 6.0 * old_r[2][i] * delta_t;
        }
        new_r[2] = (self.acceleration_function)(&new_r[0], &new_r[1], particle.mass());

        new_r
    }
}

pub struct GearPredictorCorrector<const DIM: usize> {
    acceleration_function: fn(r: &[f64; DIM], v: &[f64; DIM], mass: f64) -> [f64; DIM],
    acceleration_function_depends_on_velocity: bool,
}

impl<const DIM: usize> GearPredictorCorrector<DIM> {
    pub fn new(
        acceleration_function: fn(r: &[f64; DIM], v: &[f64; DIM], mass: f64) -> [f64; DIM],
        acceleration_function_depends_on_velocity: bool,
        particles_to_init: Vec<(&mut Particle<DIM>, Vec<[f64; DIM]>)>,
    ) -> Self {
        for (particle, derivatives) in particles_to_init {
            for derivative in derivatives {
                particle.add_derivative(derivative);
            }
        }

        Self {
            acceleration_function_depends_on_velocity,
            acceleration_function,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for GearPredictorCorrector<DIM> {
    fn calculate_step(&self, particle: &Particle<DIM>, delta_t: f64) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();

        // Predict
        let mut new_r = particle.cloned_derivatives();
        let delta_time_2 = delta_t.powi(2);
        let delta_time_3 = delta_t.powi(3);
        let delta_time_4 = delta_t.powi(4);
        let delta_time_5 = delta_t.powi(5);

        for i in 0..DIM {
            new_r[0][i] += r[1][i] * delta_t
                + 1.0 / 2.0 * delta_time_2 * r[2][i]
                + 1.0 / 6.0 * delta_time_3 * r[3][i]
                + 1.0 / 24.0 * delta_time_4 * r[4][i]
                + 1.0 / 120.0 * delta_time_5 * r[5][i];
            new_r[1][i] += r[2][i] * delta_t
                + 1.0 / 2.0 * delta_time_2 * r[3][i]
                + 1.0 / 6.0 * delta_time_3 * r[4][i]
                + 1.0 / 24.0 * delta_time_4 * r[5][i];
            new_r[2][i] += r[3][i] * delta_t
                + 1.0 / 2.0 * delta_time_2 * r[4][i]
                + 1.0 / 6.0 * delta_time_3 * r[5][i];
            new_r[3][i] += r[4][i] * delta_t + 1.0 / 2.0 * delta_time_2 * r[5][i];
            new_r[4][i] += r[5][i] * delta_t;
        }

        // Evaluate
        let new_acceleration = (self.acceleration_function)(&new_r[0], &new_r[1], particle.mass());
        let mut delta_acc = [0.0; DIM];
        for i in 0..DIM {
            delta_acc[i] = (new_acceleration[i] - new_r[2][i]) * delta_time_2 / 2.0;
        }

        let alpha_0 = if self.acceleration_function_depends_on_velocity {
            3.0 / 16.0
        } else {
            3.0 / 20.0
        };

        // Correct
        for i in 0..DIM {
            new_r[0][i] += alpha_0 * delta_acc[i];
            new_r[1][i] += 251.0 / 360.0 * delta_acc[i] / delta_t;
            new_r[2][i] += 2.0 * delta_acc[i] / delta_time_2;
            new_r[3][i] += 11.0 / 3.0 * delta_acc[i] / delta_time_3;
            new_r[4][i] += 4.0 * delta_acc[i] / delta_time_4;
            new_r[5][i] += 2.0 * delta_acc[i] / delta_time_5;
        }

        new_r
    }
}
