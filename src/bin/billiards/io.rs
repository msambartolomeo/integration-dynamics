use anyhow::Result;
use integration_dynamics::particle::Particle;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::constants::{DIM, HOLE_RADIUS, HOLE_VARIANTS};

struct RGB {
    r: f64,
    g: f64,
    b: f64,
}

impl RGB {
    fn new(r: f64, g: f64, b: f64) -> Self {
        RGB { r, g, b }
    }
}

enum Color {
    White,
    Black,
    Yellow,
    Red,
    Green,
    Blue,
    Purple,
    Orange,
    Maroon,
}

impl Color {
    fn get_rgb(&self) -> RGB {
        match self {
            Color::White => RGB::new(1.0, 1.0, 1.0),
            Color::Black => RGB::new(0.0, 0.0, 0.0),
            Color::Yellow => RGB::new(1.0, 1.0, 0.0),
            Color::Red => RGB::new(1.0, 0.0, 0.0),
            Color::Green => RGB::new(0.0, 0.5, 0.0),
            Color::Blue => RGB::new(0.0, 0.0, 1.0),
            Color::Purple => RGB::new(1.0, 0.0, 1.0),
            Color::Orange => RGB::new(1.0, 0.5, 0.0),
            Color::Maroon => RGB::new(0.5, 0.0, 0.0),
        }
    }
}

const COLORS: [Color; 16] = [
    Color::White,
    Color::Yellow,
    Color::Blue,
    Color::Red,
    Color::Purple,
    Color::Black,
    Color::Orange,
    Color::Green,
    Color::Maroon,
    Color::Yellow,
    Color::Blue,
    Color::Red,
    Color::Purple,
    Color::Orange,
    Color::Green,
    Color::Maroon,
];

pub fn output_simulation(
    file: &File,
    particles: &Vec<Particle<DIM>>,
    include_holes: bool,
) -> Result<()> {
    let mut writer = BufWriter::new(file);

    let particle_count = particles.len() + HOLE_VARIANTS.len();
    writeln!(writer, "{}", particle_count)?;
    writeln!(
        writer,
        "Properties=pos:R:{}:velo:R:{}:radius:R:1:color:R:3 pbc=\"F F\"",
        DIM, DIM
    )?;

    // NOTE: Write the particles
    for particle in particles {
        let derivatives = particle.derivatives();
        let particle_color = COLORS[particle.id() % COLORS.len()].get_rgb();

        // TODO: Generalize into DIM-dimensional, not just 2D
        writeln!(
            writer,
            "{:.12} {:.12} {:.12} {:.12} {:.4} {} {} {}",
            derivatives[0][0],
            derivatives[0][1],
            derivatives[1][0],
            derivatives[1][1],
            particle.radius(),
            particle_color.r,
            particle_color.g,
            particle_color.b
        )?;
    }

    let holes_color = Color::White.get_rgb();
    // NOTE: Write the holes
    for hole in &HOLE_VARIANTS {
        let hole_coordinates = hole.coordinates();
        let hole_radius = match include_holes {
            true => HOLE_RADIUS,
            false => 0.0,
        };

        writeln!(
            writer,
            "{:.12} {:.12} {:.12} {:.12} {:.3} {} {} {}",
            hole_coordinates[0],
            hole_coordinates[1],
            0.0,
            0.0,
            hole_radius,
            holes_color.r,
            holes_color.g,
            holes_color.b
        )?;
    }

    Ok(())
}
