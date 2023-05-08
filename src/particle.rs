#[derive(Debug)]
pub struct Particle<const DIM: usize> {
    id: usize,

    derivatives: Vec<[f64; DIM]>,

    radius: f64,
    mass: f64,
}

impl<const DIM: usize> Particle<DIM> {
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

    pub fn derivatives(&self) -> &Vec<[f64; DIM]> {
        &self.derivatives
    }

    pub fn cloned_derivatives(&self) -> Vec<[f64; DIM]> {
        self.derivatives.clone()
    }
}
