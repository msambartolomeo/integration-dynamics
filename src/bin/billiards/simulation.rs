use integration_dynamics::{
    methods::{
        Beeman, Euler, EulerMod, EulerPredictorCorrector, GearPredictorCorrector,
        IntegrationMethod, VelocityVerlet, Verlet, VerletLeapFrog,
    },
    particle::Particle,
    Integration,
};

use crate::constants::{
    self, acceleration_function, BALL_COUNT, BALL_MASS, BALL_RADIUS, BALL_SPACING_LOWER_BOUND,
    BALL_SPACING_RANGE, DIM, HOLE_RADIUS, HOLE_VARIANTS, TABLE_WIDTH,
};
use rand::Rng;

pub struct Billiards {
    balls: Vec<Particle<DIM>>,
    integration_method: Box<dyn IntegrationMethod<DIM>>,
}

impl Billiards {
    pub fn new(
        delta_t: f64,
        integration_method: &Integration,
        fixed_ball_spacing: bool,
        white_offset: f64,
        initial_velocity: [f64; DIM],
    ) -> Self {
        let mut rng = rand::thread_rng();

        let mut get_ball_spacing = move || {
            if fixed_ball_spacing {
                0.0
            } else {
                rng.gen_range(BALL_SPACING_RANGE)
            }
        };

        let mut balls = Vec::with_capacity(BALL_COUNT);
        let mut ball_id = 0;

        let white_ball = Particle::new(
            ball_id,
            [TABLE_WIDTH / 2.0, TABLE_WIDTH / 2.0 + white_offset],
            initial_velocity,
            [0.0, 0.0],
            BALL_RADIUS,
            BALL_MASS,
        );

        balls.push(white_ball);

        for (x, y) in constants::get_balls_starting_position() {
            let x_spacing = get_ball_spacing();
            let y_spacing = get_ball_spacing();
            ball_id += 1;
            let ball = Particle::new(
                ball_id,
                [x + x_spacing, y + y_spacing],
                [0.0, 0.0],
                [0.0, 0.0],
                BALL_RADIUS,
                BALL_MASS,
            );

            balls.push(ball);
        }

        for ball in &balls {
            for other_ball in &balls {
                if ball != other_ball {
                    let distance = ball.get_distance(other_ball);
                    assert!(BALL_SPACING_LOWER_BOUND <= distance);
                }
            }
        }

        let integration_method: Box<dyn IntegrationMethod<DIM>> = match integration_method {
            Integration::Euler => Box::new(Euler::new(acceleration_function, delta_t)),
            Integration::EulerMod => Box::new(EulerMod::new(acceleration_function, delta_t)),
            Integration::Verlet => {
                Box::new(Verlet::new(acceleration_function, &mut balls, delta_t))
            }
            Integration::Beeman => {
                Box::new(Beeman::new(acceleration_function, &mut balls, delta_t))
            }
            Integration::GearPredictorCorrector => {
                let particles_to_init = balls
                    .iter_mut()
                    .map(|b| (b, vec![[0.0; DIM], [0.0; DIM], [0.0; DIM]]))
                    .collect();
                Box::new(GearPredictorCorrector::new(
                    acceleration_function,
                    false,
                    particles_to_init,
                    delta_t,
                ))
            }
            Integration::VerletLeapFrog => Box::new(VerletLeapFrog::new(
                acceleration_function,
                &mut balls,
                delta_t,
            )),
            Integration::VelocityVerlet => {
                Box::new(VelocityVerlet::new(acceleration_function, delta_t))
            }
            Integration::EulerPredictorCorrector => {
                Box::new(EulerPredictorCorrector::new(acceleration_function, delta_t))
            }
        };

        Self {
            balls,
            integration_method,
        }
    }

    fn is_colliding_with_hole(particle: &Particle<DIM>) -> bool {
        let r = particle.derivatives()[0];
        let particle_radius = particle.radius();
        for hole in &HOLE_VARIANTS {
            let hole_r = hole.coordinates();
            let distance = r
                .iter()
                .zip(hole_r.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();
            if distance <= (particle_radius + HOLE_RADIUS) {
                return true;
            }
        }
        false
    }

    pub fn run(&mut self, steps: usize) -> &Vec<Particle<DIM>> {
        for _ in 0..steps {
            self.integration_method.advance_step(&mut self.balls);
            self.balls
                .retain(|particle| !Self::is_colliding_with_hole(particle));
        }
        &self.balls
    }

    pub fn balls(&self) -> &Vec<Particle<DIM>> {
        &self.balls
    }
}
