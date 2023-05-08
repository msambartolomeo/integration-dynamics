pub const DIM: usize = 2;

#[derive(Debug)]
pub struct Particle {
    id: usize,

    pub derivatives: Vec<[f64; DIM]>,

    radius: f64,
    mass: f64,
}

impl Particle {
    pub fn new(id: usize, r: [f64; DIM], v: [f64; DIM], radius: f64, mass: f64) -> Self {
        Self {
            id,
            derivatives: vec![r, v],
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
