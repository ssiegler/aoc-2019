use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::ops::{Div, Sub};

fn main() -> Result<()> {
    let masses = read_masses(&determine_input_filename())?;

    let sum: u32 = masses.iter().map(fuel_required).sum();
    println!("{}", sum);
    Ok(())
}

fn determine_input_filename() -> String {
    env::args().nth(1).expect("Expected input filename")
}

fn read_masses(filename: &str) -> Result<Vec<u32>> {
    let file = File::open(filename)?;
    let file = BufReader::new(file);
    Ok(file
        .lines()
        .map(|line| {
            line.expect("Could not read line")
                .parse::<u32>()
                .expect("Could not parse mass")
        })
        .collect())
}

fn fuel_required(mass: &u32) -> u32 {
    mass.div(3).sub(2)
}

#[cfg(test)]
mod tests {
    use super::fuel_required;

    #[test]
    fn mass_of_12_needs_2_fuel() {
        assert_eq!(2, fuel_required(&12));
    }

    #[test]
    fn mass_of_14_needs_2_fuel() {
        assert_eq!(2, fuel_required(&14));
    }

    #[test]
    fn mass_of_1969_needs_654_fuel() {
        assert_eq!(654, fuel_required(&1969));
    }

    #[test]
    fn mass_of_100756_needs_33583_fuel() {
        assert_eq!(33583, fuel_required(&100756));
    }
}
