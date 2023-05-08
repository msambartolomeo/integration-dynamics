use integration_dynamics::particle::Particle;

pub const TIME_STEP: f64 = 5.0;
pub const DIM: usize = 1;

pub const PARTICLE_MASS: f64 = 70.0;
pub const AMPLITUDE: f64 = 1.0;

pub const RESTORING_FORCE_CONSTANT: f64 = 10e4;
pub const AMORTIGUATION_CONSTANT: f64 = 10e2;
pub const INITIAL_POSITION: [f64; DIM] = [1.0];
pub const INITIAL_VELOCITY: [f64; DIM] =
    [-AMPLITUDE * AMORTIGUATION_CONSTANT / (2.0 * PARTICLE_MASS)];

pub fn acceleration_function(particle: &Particle<DIM>) -> f64 {
    let r = particle.derivatives();

    let restoring_force = -RESTORING_FORCE_CONSTANT * r[0][0];
    let amortiguation_force = -AMORTIGUATION_CONSTANT * r[1][0];

    (restoring_force + amortiguation_force) / particle.mass()
}
