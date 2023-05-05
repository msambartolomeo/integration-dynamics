use crate::Particle;

pub struct Euler;

impl Euler {
    pub fn method(particle: &mut Particle, delta_t: f64) {
        for i in 0..2 {
            particle.r[i] += delta_t * particle.v[i] + delta_t.powf(2.0) / 2.0 * particle.a[i];
            particle.v[i] += delta_t * particle.a[i];
        }
    }
}

pub struct EulerMod;

impl EulerMod {
    pub fn method(particle: &mut Particle, delta_t: f64) {
        for i in 0..2 {
            particle.v[i] += delta_t * particle.a[i];
            particle.r[i] += delta_t * particle.v[i] + delta_t.powf(2.0) / 2.0 * particle.a[i];
        }
    }
}
