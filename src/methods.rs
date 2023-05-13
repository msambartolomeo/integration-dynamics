use crate::particle::Particle;

pub trait IntegrationMethod<const DIM: usize> {
    fn calculate_step(&self, particle: &Particle<DIM>, others: &[Particle<DIM>])
        -> Vec<[f64; DIM]>;
    fn advance_step(&self, particles: &mut [Particle<DIM>]) {
        let mut derivatives = Vec::new();

        for particle in particles.iter() {
            derivatives.push(self.calculate_step(particle, particles));
        }

        for (i, particle) in particles.iter_mut().enumerate() {
            let old = particle.set_derivatives(std::mem::take(&mut derivatives[i]));
            particle.set_prev_derivatives(old);
        }
    }
}

pub struct Euler<const DIM: usize> {
    acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
    delta_t: f64,
}

impl<const DIM: usize> Euler<DIM> {
    pub fn new(
        acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
        delta_t: f64,
    ) -> Self {
        Self {
            acceleration_function,
            delta_t,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for Euler<DIM> {
    fn calculate_step(
        &self,
        particle: &Particle<DIM>,
        others: &[Particle<DIM>],
    ) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let mut new_r = particle.cloned_derivatives();

        for i in 0..DIM {
            new_r[0][i] += self.delta_t * r[1][i] + self.delta_t.powi(2) / 2.0 * r[2][i];
            new_r[1][i] += self.delta_t * r[2][i];
        }

        let new_p = Particle::new(
            particle.id(),
            new_r[0],
            new_r[1],
            new_r[2],
            particle.radius(),
            particle.mass(),
        );
        new_r[2] = (self.acceleration_function)(&new_p, others);

        new_r
    }
}

pub struct EulerMod<const DIM: usize> {
    acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
    delta_t: f64,
}

impl<const DIM: usize> EulerMod<DIM> {
    pub fn new(
        acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
        delta_t: f64,
    ) -> Self {
        Self {
            acceleration_function,
            delta_t,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for EulerMod<DIM> {
    fn calculate_step(
        &self,
        particle: &Particle<DIM>,
        others: &[Particle<DIM>],
    ) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let mut new_r = particle.cloned_derivatives();

        for i in 0..DIM {
            new_r[1][i] += self.delta_t * r[2][i];
            new_r[0][i] += self.delta_t * new_r[1][i] + self.delta_t.powi(2) / 2.0 * r[2][i];
        }

        let new_p = Particle::new(
            particle.id(),
            new_r[0],
            new_r[1],
            new_r[2],
            particle.radius(),
            particle.mass(),
        );
        new_r[2] = (self.acceleration_function)(&new_p, others);

        new_r
    }
}

pub struct Verlet<const DIM: usize> {
    acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
    delta_t: f64,
}

impl<const DIM: usize> Verlet<DIM> {
    pub fn new(
        acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
        particles_to_init: &mut [Particle<DIM>],
        delta_t: f64,
    ) -> Self {
        let euler = Euler::new(acceleration_function, -delta_t);

        let mut prev_derivatives = Vec::new();
        for particle in particles_to_init.iter() {
            prev_derivatives.push(euler.calculate_step(particle, particles_to_init));
        }

        for (i, particle) in particles_to_init.iter_mut().enumerate() {
            particle.set_prev_derivatives(std::mem::take(&mut prev_derivatives[i]));
        }

        Self {
            acceleration_function,
            delta_t,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for Verlet<DIM> {
    fn calculate_step(
        &self,
        particle: &Particle<DIM>,
        others: &[Particle<DIM>],
    ) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let old_r = particle.prev_derivatives();
        let mut new_r = particle.cloned_derivatives();

        for i in 0..DIM {
            new_r[0][i] *= 2.0;
            new_r[0][i] += -old_r[0][i] + self.delta_t.powi(2) * r[2][i];

            new_r[1][i] = (new_r[0][i] - old_r[0][i]) / (2.0 * self.delta_t);
        }

        let new_p = Particle::new(
            particle.id(),
            new_r[0],
            new_r[1],
            new_r[2],
            particle.radius(),
            particle.mass(),
        );
        new_r[2] = (self.acceleration_function)(&new_p, others);

        new_r
    }
}

pub struct VerletLeapFrog<const DIM: usize> {
    acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
    delta_t: f64,
}

impl<const DIM: usize> VerletLeapFrog<DIM> {
    pub fn new(
        acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
        particles_to_init: &mut [Particle<DIM>],
        delta_t: f64,
    ) -> Self {
        let euler = Euler::new(acceleration_function, -delta_t / 2.0);

        let mut prev_derivatives = Vec::new();
        for particle in particles_to_init.iter() {
            prev_derivatives.push(euler.calculate_step(particle, particles_to_init));
        }

        for (i, particle) in particles_to_init.iter_mut().enumerate() {
            particle.set_prev_derivatives(std::mem::take(&mut prev_derivatives[i]));
        }
        Self {
            acceleration_function,
            delta_t,
        }
    }

    fn get_v_half_step(&self, particle: &Particle<DIM>) -> [f64; DIM] {
        let r = particle.derivatives();
        let old_r = particle.prev_derivatives();

        let mut v_half_step = [0.0; DIM];

        for i in 0..DIM {
            v_half_step[i] = old_r[1][i] + self.delta_t * r[2][i];
        }

        v_half_step
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for VerletLeapFrog<DIM> {
    fn calculate_step(
        &self,
        particle: &Particle<DIM>,
        others: &[Particle<DIM>],
    ) -> Vec<[f64; DIM]> {
        let old_r = particle.prev_derivatives();
        let mut new_r = particle.cloned_derivatives();

        let v_half_step = self.get_v_half_step(particle);

        for i in 0..DIM {
            new_r[0][i] += self.delta_t * v_half_step[i];

            new_r[1][i] = (old_r[1][i] - v_half_step[i]) / 2.0;
        }

        let new_p = Particle::new(
            particle.id(),
            new_r[0],
            new_r[1],
            new_r[2],
            particle.radius(),
            particle.mass(),
        );
        new_r[2] = (self.acceleration_function)(&new_p, others);

        new_r
    }

    fn advance_step(&self, particles: &mut [Particle<DIM>]) {
        let mut derivatives = Vec::new();

        for particle in particles.iter() {
            derivatives.push(self.calculate_step(particle, particles));
        }

        for (i, particle) in particles.iter_mut().enumerate() {
            let mut old = particle.set_derivatives(std::mem::take(&mut derivatives[i]));

            // NOTE: Use v(t + delta_t/2) for previous instead of v(t)
            old[1] = self.get_v_half_step(particle);
            particle.set_prev_derivatives(old);
        }
    }
}

pub struct VelocityVerlet<const DIM: usize> {
    acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
    delta_t: f64,
}

impl<const DIM: usize> VelocityVerlet<DIM> {
    pub fn new(
        acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
        delta_t: f64,
    ) -> Self {
        Self {
            acceleration_function,
            delta_t,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for VelocityVerlet<DIM> {
    fn calculate_step(
        &self,
        particle: &Particle<DIM>,
        others: &[Particle<DIM>],
    ) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let mut new_r = particle.cloned_derivatives();

        let new_p = Particle::new(
            particle.id(),
            new_r[0],
            new_r[1],
            new_r[2],
            particle.radius(),
            particle.mass(),
        );
        new_r[2] = (self.acceleration_function)(&new_p, others);

        for i in 0..DIM {
            new_r[0][i] += self.delta_t * r[1][i] + self.delta_t.powi(2) * r[2][i];
            new_r[1][i] += self.delta_t / 2.0 * (r[2][i] + new_r[2][i]);
        }

        new_r
    }
}

pub struct Beeman<const DIM: usize> {
    acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
    delta_t: f64,
}

impl<const DIM: usize> Beeman<DIM> {
    pub fn new(
        acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
        particles_to_init: &mut [Particle<DIM>],
        delta_t: f64,
    ) -> Self {
        let euler = Euler::new(acceleration_function, -delta_t);

        let mut prev_derivatives = Vec::new();
        for particle in particles_to_init.iter() {
            prev_derivatives.push(euler.calculate_step(particle, particles_to_init));
        }

        for (i, particle) in particles_to_init.iter_mut().enumerate() {
            particle.set_prev_derivatives(std::mem::take(&mut prev_derivatives[i]));
        }

        Self {
            acceleration_function,
            delta_t,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for Beeman<DIM> {
    fn calculate_step(
        &self,
        particle: &Particle<DIM>,
        others: &[Particle<DIM>],
    ) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let old_r = particle.prev_derivatives();
        let mut new_r = particle.cloned_derivatives();

        for i in 0..DIM {
            new_r[0][i] += r[1][i] * self.delta_t + 2.0 / 3.0 * r[2][i] * self.delta_t.powi(2)
                - 1.0 / 6.0 * old_r[2][i] * self.delta_t.powi(2);

            new_r[1][i] += 1.0 / 3.0 * new_r[2][i] * self.delta_t
                + 5.0 / 6.0 * r[2][i] * self.delta_t
                - 1.0 / 6.0 * old_r[2][i] * self.delta_t;
        }

        let new_p = Particle::new(
            particle.id(),
            new_r[0],
            new_r[1],
            new_r[2],
            particle.radius(),
            particle.mass(),
        );
        new_r[2] = (self.acceleration_function)(&new_p, others);

        new_r
    }
}

pub struct EulerPredictorCorrector<const DIM: usize> {
    acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
    delta_t: f64,
}

impl<const DIM: usize> EulerPredictorCorrector<DIM> {
    pub fn new(
        acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
        delta_t: f64,
    ) -> Self {
        Self {
            acceleration_function,
            delta_t,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for EulerPredictorCorrector<DIM> {
    fn calculate_step(
        &self,
        particle: &Particle<DIM>,
        others: &[Particle<DIM>],
    ) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();
        let mut new_r = particle.cloned_derivatives();

        // Predict
        for i in 0..DIM {
            new_r[1][i] += r[2][i] * self.delta_t;
            new_r[0][i] += r[1][i] * self.delta_t;
        }

        // Evalate
        let new_p = Particle::new(
            particle.id(),
            new_r[0],
            new_r[1],
            new_r[2],
            particle.radius(),
            particle.mass(),
        );
        new_r[2] = (self.acceleration_function)(&new_p, others);

        // Correct
        for i in 0..DIM {
            new_r[1][i] = r[1][i] + new_r[2][i] * self.delta_t;
            new_r[0][i] = r[0][i] + new_r[1][i] * self.delta_t;
        }

        new_r
    }
}

pub struct GearPredictorCorrector<const DIM: usize> {
    acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
    acceleration_function_depends_on_velocity: bool,
    delta_t: f64,
}

impl<const DIM: usize> GearPredictorCorrector<DIM> {
    pub fn new(
        acceleration_function: fn(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM],
        acceleration_function_depends_on_velocity: bool,
        particles_to_init: Vec<(&mut Particle<DIM>, Vec<[f64; DIM]>)>,
        delta_t: f64,
    ) -> Self {
        for (particle, derivatives) in particles_to_init {
            for derivative in derivatives {
                particle.add_derivative(derivative);
            }
        }

        Self {
            acceleration_function,
            acceleration_function_depends_on_velocity,
            delta_t,
        }
    }
}

impl<const DIM: usize> IntegrationMethod<DIM> for GearPredictorCorrector<DIM> {
    fn calculate_step(
        &self,
        particle: &Particle<DIM>,
        others: &[Particle<DIM>],
    ) -> Vec<[f64; DIM]> {
        let r = particle.derivatives();

        // Predict
        let mut new_r = particle.cloned_derivatives();
        let delta_time_2 = self.delta_t.powi(2);
        let delta_time_3 = self.delta_t.powi(3);
        let delta_time_4 = self.delta_t.powi(4);
        let delta_time_5 = self.delta_t.powi(5);

        for i in 0..DIM {
            new_r[0][i] += r[1][i] * self.delta_t
                + 1.0 / 2.0 * delta_time_2 * r[2][i]
                + 1.0 / 6.0 * delta_time_3 * r[3][i]
                + 1.0 / 24.0 * delta_time_4 * r[4][i]
                + 1.0 / 120.0 * delta_time_5 * r[5][i];
            new_r[1][i] += r[2][i] * self.delta_t
                + 1.0 / 2.0 * delta_time_2 * r[3][i]
                + 1.0 / 6.0 * delta_time_3 * r[4][i]
                + 1.0 / 24.0 * delta_time_4 * r[5][i];
            new_r[2][i] += r[3][i] * self.delta_t
                + 1.0 / 2.0 * delta_time_2 * r[4][i]
                + 1.0 / 6.0 * delta_time_3 * r[5][i];
            new_r[3][i] += r[4][i] * self.delta_t + 1.0 / 2.0 * delta_time_2 * r[5][i];
            new_r[4][i] += r[5][i] * self.delta_t;
        }

        // Evaluate
        let new_p = Particle::new(
            particle.id(),
            new_r[0],
            new_r[1],
            new_r[2],
            particle.radius(),
            particle.mass(),
        );
        let new_acceleration = (self.acceleration_function)(&new_p, others);

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
            new_r[1][i] += 251.0 / 360.0 * delta_acc[i] / self.delta_t;
            new_r[2][i] += 2.0 * delta_acc[i] / delta_time_2;
            new_r[3][i] += 11.0 / 3.0 * delta_acc[i] / delta_time_3;
            new_r[4][i] += 4.0 * delta_acc[i] / delta_time_4;
            new_r[5][i] += 2.0 * delta_acc[i] / delta_time_5;
        }

        new_r
    }
}
