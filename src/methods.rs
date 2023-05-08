use crate::{particle::DIM, Particle};

pub struct Euler;

impl Euler {
    pub fn method(particle: &Particle, delta_t: f64) -> Vec<[f64; DIM]> {
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

impl EulerMod {
    pub fn method(particle: &Particle, delta_t: f64) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let mut new_r = particle.cloned_derivatives();

        for i in 0..DIM {
            new_r[1][i] += delta_t * r[2][i];
            new_r[0][i] += delta_t * r[1][i] + delta_t.powf(2.0) / 2.0 * r[2][i];
        }

        new_r
    }
}
