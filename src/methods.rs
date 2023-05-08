use crate::particle::Particle;

pub trait IntegrationMethod<const DIM: usize> {
    fn calculate_step(&self, particle: &Particle<DIM>, delta_t: f64) -> Vec<[f64; DIM]>;
}

pub struct Euler;
impl<const DIM: usize> IntegrationMethod<DIM> for Euler {
    fn calculate_step(&self, particle: &Particle<DIM>, delta_t: f64) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let mut new_r = particle.cloned_derivatives();

        for i in 0..DIM {
            new_r[0][i] += delta_t * r[1][i] + delta_t.powf(2.0) / 2.0 * r[2][i];
            new_r[1][i] += delta_t * r[2][i];
        }

        new_r
    }
}

pub struct EulerMod;
impl<const DIM: usize> IntegrationMethod<DIM> for EulerMod {
    fn calculate_step(&self, particle: &Particle<DIM>, delta_t: f64) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let mut new_r = particle.cloned_derivatives();

        for i in 0..DIM {
            new_r[1][i] += delta_t * r[2][i];
            new_r[0][i] += delta_t * r[1][i] + delta_t.powf(2.0) / 2.0 * r[2][i];
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
