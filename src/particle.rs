#[derive(Debug)]
pub struct Particle {
    id: usize,

    pub r: [f64; 2],
    pub v: [f64; 2],
    pub a: [f64; 2],

    radius: f64,
    mass: f64,
}

impl Particle {
    pub fn new(id: usize, r: (f64, f64), v: (f64, f64), radius: f64, mass: f64) -> Self {
        Self {
            id,
            r: [r.0, r.1],
            v: [v.0, v.1],
            a: [0.0, 0.0],
            radius,
            mass,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }
}
