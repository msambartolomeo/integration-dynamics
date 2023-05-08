use crate::particle::Particle;

pub trait IntegrationMethod<const DIM: usize> {
    fn calculate_step(&self, particle: &Particle<DIM>, delta_t: f64) -> Vec<[f64; DIM]>;
}

pub struct Euler<const DIM: usize> {
    acceleration_function: fn(&Particle<DIM>) -> f64,
}

impl<const DIM: usize> Euler<DIM> {
    pub fn new(acceleration_function: fn(&Particle<DIM>) -> f64) -> Self {
        Self {
            acceleration_function,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for Euler<DIM> {
    fn calculate_step(&self, particle: &Particle<DIM>, delta_t: f64) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let mut new_r = particle.cloned_derivatives();
        let new_acceleration = (self.acceleration_function)(&particle);

        for i in 0..DIM {
            new_r[0][i] += delta_t * r[1][i] + delta_t.powf(2.0) / 2.0 * r[2][i];
            new_r[1][i] += delta_t * r[2][i];
            new_r[2][i] = new_acceleration;
        }

        new_r
    }
}

pub struct EulerMod<const DIM: usize> {
    acceleration_function: fn(&Particle<DIM>) -> f64,
}

impl<const DIM: usize> EulerMod<DIM> {
    pub fn new(acceleration_function: fn(&Particle<DIM>) -> f64) -> Self {
        Self {
            acceleration_function,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for EulerMod<DIM> {
    fn calculate_step(&self, particle: &Particle<DIM>, delta_t: f64) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let mut new_r = particle.cloned_derivatives();
        let new_acceleration = (self.acceleration_function)(&particle);

        for i in 0..DIM {
            new_r[1][i] += delta_t * r[2][i];
            new_r[0][i] += delta_t * r[1][i] + delta_t.powf(2.0) / 2.0 * r[2][i];
            new_r[2][i] = new_acceleration;
        }

        new_r
    }
}

pub struct Verlet;
impl<const DIM: usize> IntegrationMethod<DIM> for Verlet {
    fn calculate_step(&self, particle: &Particle<DIM>, delta_t: f64) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let mut new_r = particle.cloned_derivatives();

        for i in 0..DIM {
            new_r[0][1] = 2.0 * new_r[0][1];
        }
        todo!();
    }
}
