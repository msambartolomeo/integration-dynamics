#[derive(Debug, PartialEq)]
pub struct Particle<const DIM: usize> {
    id: usize,

    derivatives: Vec<[f64; DIM]>,
    prev_derivatives: Vec<[f64; DIM]>,

    radius: f64,
    mass: f64,
}

impl<const DIM: usize> Particle<DIM> {
    #[must_use]
    pub fn new(
        id: usize,
        r: [f64; DIM],
        v: [f64; DIM],
        a: [f64; DIM],
        radius: f64,
        mass: f64,
    ) -> Self {
        Self {
            id,
            derivatives: vec![r, v, a],
            prev_derivatives: vec![r, v, a],
            radius,
            mass,
        }
    }

    #[must_use]
    pub fn id(&self) -> usize {
        self.id
    }

    #[must_use]
    pub fn radius(&self) -> f64 {
        self.radius
    }

    #[must_use]
    pub fn mass(&self) -> f64 {
        self.mass
    }

    #[must_use]
    pub fn derivatives(&self) -> &Vec<[f64; DIM]> {
        &self.derivatives
    }

    #[must_use]
    pub fn get_distance(&self, other: &Self) -> f64 {
        let mut distance = 0.0;

        for i in 0..DIM {
            distance += (self.derivatives[0][i] - other.derivatives[0][i]).powi(2);
        }

        distance.sqrt()
    }

    pub(crate) fn prev_derivatives(&self) -> &Vec<[f64; DIM]> {
        &self.prev_derivatives
    }

    pub(crate) fn set_prev_derivatives(&mut self, derivatives: Vec<[f64; DIM]>) {
        self.prev_derivatives = derivatives;
    }

    pub(crate) fn set_derivatives(&mut self, derivatives: Vec<[f64; DIM]>) -> Vec<[f64; DIM]> {
        std::mem::replace(&mut self.derivatives, derivatives)
    }

    pub(crate) fn cloned_derivatives(&self) -> Vec<[f64; DIM]> {
        self.derivatives.clone()
    }

    pub(crate) fn add_derivative(&mut self, derivative: [f64; DIM]) {
        self.derivatives.push(derivative);
    }
}
