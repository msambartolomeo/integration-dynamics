use integration_dynamics::{
    methods::{Beeman, Euler, EulerMod, GearPredictorCorrector, IntegrationMethod, Verlet},
    particle::Particle,
    Integration,
};

use crate::constants::{
    self, acceleration_function, BALL_COUNT, BALL_MASS, BALL_RADIUS, BALL_SPACING_LOWER_BOUND,
    BALL_SPACING_RANGE, DIM, TABLE_WIDTH,
};
use rand::Rng;

pub struct Billiards {
    balls: Vec<Option<Particle<DIM>>>,
    integration_method: Box<dyn IntegrationMethod<DIM>>,
}

impl Billiards {
    pub fn new(
        delta_t: f64,
        integration_method: Integration,
        fixed_ball_spacing: bool,
        white_offset: f64,
        initial_velocity: f64,
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

        let white_ball = Particle::new(
            [TABLE_WIDTH / 2.0, TABLE_WIDTH / 2.0 + white_offset],
            [0.0, initial_velocity],
            [0.0, 0.0],
            BALL_RADIUS,
            BALL_MASS,
        );

        balls.push(Some(white_ball));

        for (x, y) in constants::get_balls_starting_position() {
            let x_spacing = get_ball_spacing();
            let y_spacing = get_ball_spacing();
            let ball = Particle::new(
                [x + x_spacing, y + y_spacing],
                [0.0, 0.0],
                [0.0, 0.0],
                BALL_RADIUS,
                BALL_MASS,
            );

            balls.push(Some(ball));
        }

        for ball in balls.iter().flatten() {
            for other_ball in balls.iter().flatten() {
                if ball != other_ball {
                    let distance = ball.get_distance(other_ball);
                    assert!(BALL_SPACING_LOWER_BOUND <= distance);
                }
            }
        }

        let integration_method: Box<dyn IntegrationMethod<DIM>> = match integration_method {
            Integration::Euler => Box::new(Euler::new(acceleration_function, delta_t)),
            Integration::EulerMod => Box::new(EulerMod::new(acceleration_function, delta_t)),
            Integration::Verlet => Box::new(Verlet::new(
                acceleration_function,
                &mut balls.iter_mut().flatten().collect::<Vec<_>>(),
                delta_t,
            )),
            Integration::Beeman => Box::new(Beeman::new(
                acceleration_function,
                &mut balls.iter_mut().flatten().collect::<Vec<_>>(),
                delta_t,
            )),
            Integration::GearPredictorCorrector => {
                let particles_to_init = balls
                    .iter_mut()
                    .flatten()
                    .map(|b| (b, vec![[0.0; DIM], [0.0; DIM], [0.0; DIM]]))
                    .collect();
                Box::new(GearPredictorCorrector::new(
                    acceleration_function,
                    false,
                    particles_to_init,
                    delta_t,
                ))
            }
        };

        Self {
            balls,
            integration_method,
        }
    }
}
