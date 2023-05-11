use std::ops::RangeInclusive;

pub const DIM: usize = 2;

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

pub const WALL_VARIANTS: [Wall; 4] = [Wall::Top, Wall::Bottom, Wall::Left, Wall::Right];

pub enum Hole {
    BottomLeft,
    BottomMiddle,
    BottomRight,
    TopLeft,
    TopMiddle,
    TopRight,
}

impl Hole {
    pub fn coordinates(&self) -> (f64, f64) {
        match self {
            Hole::BottomLeft => (0.0, 0.0),
            Hole::BottomMiddle => (TABLE_LENGTH / 2.0, 0.0),
            Hole::BottomRight => (TABLE_LENGTH, 0.0),
            Hole::TopLeft => (0.0, TABLE_WIDTH),
            Hole::TopMiddle => (TABLE_LENGTH / 2.0, TABLE_WIDTH),
            Hole::TopRight => (TABLE_LENGTH, TABLE_WIDTH),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Wall {
    Top,
    Bottom,
    Left,
    Right,
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

pub fn acceleration_function(r: &[f64; DIM], _v: &[f64; DIM], mass: f64) -> [f64; DIM] {
    let mut acceleration = [0.0; DIM];

    for i in 0..DIM {
        acceleration[i] = 3.0; // TODO: Implement acceleration function
    }

    acceleration
}
