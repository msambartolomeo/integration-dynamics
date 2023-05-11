use std::vec;

#[derive(Debug)]
pub struct Particle<const DIM: usize> {
    derivatives: Vec<[f64; DIM]>,
    prev_derivatives: Vec<[f64; DIM]>,

    radius: f64,
    mass: f64,
}

impl<const DIM: usize> Particle<DIM> {
    pub fn new(r: [f64; DIM], v: [f64; DIM], a: [f64; DIM], radius: f64, mass: f64) -> Self {
        Self {
            derivatives: vec![r, v, a],
            prev_derivatives: vec![],
            radius,
            mass,
        }
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

    pub fn prev_derivatives(&self) -> &Vec<[f64; DIM]> {
        &self.prev_derivatives
    }

    pub fn set_prev_derivatives(&mut self, derivatives: Vec<[f64; DIM]>) {
        self.prev_derivatives = derivatives;
    }

    pub fn set_derivatives(&mut self, derivatives: Vec<[f64; DIM]>) {
        std::mem::swap(&mut self.prev_derivatives, &mut self.derivatives);
        self.derivatives = derivatives;
    }

    pub fn cloned_derivatives(&self) -> Vec<[f64; DIM]> {
        self.derivatives.clone()
    }

    pub fn add_derivative(&mut self, derivative: [f64; DIM]) {
        self.derivatives.push(derivative);
    }
}
