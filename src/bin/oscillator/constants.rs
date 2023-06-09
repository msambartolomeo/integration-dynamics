use integration_dynamics::particle::Particle;

pub const DIM: usize = 1;

pub const PARTICLE_MASS: f64 = 70.0;
pub const AMPLITUDE: f64 = 1.0;

pub const RESTORING_FORCE_CONSTANT: f64 = 1e4;
pub const AMORTIGUATION_CONSTANT: f64 = 1e2;
pub const INITIAL_POSITION: [f64; DIM] = [1.0];
pub const INITIAL_VELOCITY: [f64; DIM] =
    [-AMPLITUDE * AMORTIGUATION_CONSTANT / (2.0 * PARTICLE_MASS)];
pub const INITIAL_ACCELERATION: [f64; DIM] = [(-RESTORING_FORCE_CONSTANT * INITIAL_POSITION[0]
    - AMORTIGUATION_CONSTANT * INITIAL_VELOCITY[0])
    / PARTICLE_MASS];
pub const INITIAL_THIRD_DERIVATIVE: [f64; DIM] = [(-RESTORING_FORCE_CONSTANT
    * INITIAL_VELOCITY[0]
    - AMORTIGUATION_CONSTANT * INITIAL_ACCELERATION[0])
    / PARTICLE_MASS];
pub const INITIAL_FOURTH_DERIVATIVE: [f64; DIM] = [(-RESTORING_FORCE_CONSTANT
    * INITIAL_ACCELERATION[0]
    - AMORTIGUATION_CONSTANT * INITIAL_THIRD_DERIVATIVE[0])
    / PARTICLE_MASS];
pub const INITIAL_FIFTH_DERIVATIVE: [f64; DIM] = [(-RESTORING_FORCE_CONSTANT
    * INITIAL_THIRD_DERIVATIVE[0]
    - AMORTIGUATION_CONSTANT * INITIAL_FOURTH_DERIVATIVE[0])
    / PARTICLE_MASS];

pub fn acceleration_function(particle: &Particle<DIM>, _others: &[Particle<DIM>]) -> [f64; DIM] {
    let mut acceleration = [0.0; DIM];
    let r = particle.derivatives();

    for i in 0..DIM {
        acceleration[i] = (-RESTORING_FORCE_CONSTANT * r[0][i] - AMORTIGUATION_CONSTANT * r[1][i])
            / particle.mass();
    }

    acceleration
}

const A: f64 = -AMORTIGUATION_CONSTANT / (2.0 * PARTICLE_MASS);
const B: f64 = RESTORING_FORCE_CONSTANT / PARTICLE_MASS
    - AMORTIGUATION_CONSTANT * AMORTIGUATION_CONSTANT / (4.0 * PARTICLE_MASS * PARTICLE_MASS);

pub fn analytic_solution(t: f64) -> f64 {
    AMPLITUDE * (A * t).exp() * (B.sqrt() * t).cos()
}
