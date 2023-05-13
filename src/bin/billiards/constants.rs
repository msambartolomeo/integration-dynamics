use std::ops::RangeInclusive;

use integration_dynamics::particle::Particle;

pub const DIM: usize = 2;
const RESTORING_FORCE_CONSTANT: f64 = 1e4;

pub const INITIAL_WHITE_BALL_VELOCITY: [f64; DIM] = [1.0, 0.0];
pub const TABLE_WIDTH: f64 = 1.12;
pub const TABLE_LENGTH: f64 = 2.24;
pub const BALL_COUNT: usize = 16;
pub const BALL_MASS: f64 = 0.165;
pub const BALL_RADIUS: f64 = 0.057 / 2.0;
pub const HOLE_RADIUS: f64 = BALL_RADIUS * 2.0;
pub const BALL_SPACING_LOWER_BOUND: f64 = 2e-4;
pub const BALL_SPACING_UPPER_BOUND: f64 = 3e-4;
pub const BALL_SPACING_NOISE: f64 = (BALL_SPACING_UPPER_BOUND - BALL_SPACING_LOWER_BOUND) / 4.0;
pub const BALL_SPACING_RANGE: RangeInclusive<f64> = -BALL_SPACING_NOISE..=BALL_SPACING_NOISE;
const BALL_RADIUS_WITH_SPACING: f64 =
    BALL_RADIUS + (BALL_SPACING_UPPER_BOUND + BALL_SPACING_LOWER_BOUND) / 4.0;

pub const HOLE_VARIANTS: [Hole; 6] = [
    Hole::BottomLeft,
    Hole::BottomMiddle,
    Hole::BottomRight,
    Hole::TopLeft,
    Hole::TopMiddle,
    Hole::TopRight,
];

pub enum Hole {
    BottomLeft,
    BottomMiddle,
    BottomRight,
    TopLeft,
    TopMiddle,
    TopRight,
}

impl Hole {
    pub fn coordinates(&self) -> [f64; DIM] {
        match self {
            Hole::BottomLeft => [0.0, 0.0],
            Hole::BottomMiddle => [TABLE_LENGTH / 2.0, 0.0],
            Hole::BottomRight => [TABLE_LENGTH, 0.0],
            Hole::TopLeft => [0.0, TABLE_WIDTH],
            Hole::TopMiddle => [TABLE_LENGTH / 2.0, TABLE_WIDTH],
            Hole::TopRight => [TABLE_LENGTH, TABLE_WIDTH],
        }
    }
}

pub fn get_balls_starting_position() -> Vec<(f64, f64)> {
    let mut positions = Vec::with_capacity(BALL_COUNT - 1);

    for (row, y_coordinates) in Y_COORDINATES_PER_ROW.iter().enumerate() {
        let x = 3f64.sqrt() * BALL_RADIUS_WITH_SPACING * row as f64;

        for y in y_coordinates.iter().flatten() {
            positions.push((TABLE_LENGTH - TABLE_WIDTH / 2.0 + x, TABLE_WIDTH / 2.0 + y));
        }
    }

    positions
}

const Y_COORDINATES_PER_ROW: [[Option<f64>; 5]; 5] = [
    [Some(0.0), None, None, None, None],
    [
        Some(-BALL_RADIUS_WITH_SPACING),
        Some(BALL_RADIUS_WITH_SPACING),
        None,
        None,
        None,
    ],
    [
        Some(-2.0 * BALL_RADIUS_WITH_SPACING),
        Some(0.0),
        Some(2.0 * BALL_RADIUS_WITH_SPACING),
        None,
        None,
    ],
    [
        Some(-3.0 * BALL_RADIUS_WITH_SPACING),
        Some(-BALL_RADIUS_WITH_SPACING),
        Some(BALL_RADIUS_WITH_SPACING),
        Some(3.0 * BALL_RADIUS_WITH_SPACING),
        None,
    ],
    [
        Some(-4.0 * BALL_RADIUS_WITH_SPACING),
        Some(-2.0 * BALL_RADIUS_WITH_SPACING),
        Some(0.0),
        Some(2.0 * BALL_RADIUS_WITH_SPACING),
        Some(4.0 * BALL_RADIUS_WITH_SPACING),
    ],
];

const DIMENSION_MAX_LENGHTS: [f64; DIM] = [TABLE_LENGTH, TABLE_WIDTH];

pub fn acceleration_function(particle: &Particle<DIM>, others: &[Particle<DIM>]) -> [f64; DIM] {
    let derivatives = particle.derivatives();
    let mut forces = [0.0; DIM];

    // Add particle collisions
    for other in others {
        if particle.id() == other.id() {
            continue;
        }

        let other_pos = other.derivatives()[0];
        let mut delta_r = [0.0; DIM];
        for i in 0..DIM {
            delta_r[i] = other_pos[i] - derivatives[0][i];
        }

        let euclidean_distance = delta_r.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();

        let radius_sum = particle.radius() + other.radius();
        if radius_sum < euclidean_distance {
            continue;
        }

        for i in 0..DIM {
            forces[i] += RESTORING_FORCE_CONSTANT
                // Distance between centers minus the sum of the radii
                * (euclidean_distance - radius_sum)
                // Unit vector in the direction of the other particle
                * (delta_r[i] / euclidean_distance);
        }
    }

    // Add wall collisions
    for i in 0..DIM {
        // Left and bottom walls
        if derivatives[0][i] <= particle.radius() {
            forces[i] += RESTORING_FORCE_CONSTANT * (particle.radius() - derivatives[0][i]);
        }
        // Right and top walls
        else if derivatives[0][i] >= DIMENSION_MAX_LENGHTS[i] - particle.radius() {
            forces[i] += RESTORING_FORCE_CONSTANT
                * (DIMENSION_MAX_LENGHTS[i] - particle.radius() - derivatives[0][i]);
        }
    }

    forces.map(|f| f / particle.mass())
}
