use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::Result;

pub fn output_simulation(path: &str, steps: &[(f64, f64)]) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for step in steps {
        writeln!(writer, "3")?;
        writeln!(writer, "Properties=pos:R:1:velo:R:1:radius:R:1",)?;
        writeln!(writer, "{:.8} {:.8} 0.1", step.0, step.1)?;
        writeln!(writer, "-1.5, 0.0 0.0")?;
        writeln!(writer, "1.5, 0.0 0.0")?;
    }

    Ok(())
}

pub struct Data {
    time: f64,
    numeric_position: f64,
    analitic_position: f64,
    mean_square_error: f64,
}

impl Data {
    pub fn new(
        time: f64,
        numeric_position: f64,
        analitic_position: f64,
        mean_square_error: f64,
    ) -> Self {
        Data {
            time,
            numeric_position,
            analitic_position,
            mean_square_error,
        }
    }
}

pub fn output_data(path: &str, data: &[Data]) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for value in data {
        writeln!(
            writer,
            "{} {} {} {}",
            value.time, value.numeric_position, value.analitic_position, value.mean_square_error
        )?;
    }

    Ok(())
}
