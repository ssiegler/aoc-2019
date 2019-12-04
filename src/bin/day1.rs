#[macro_use]
extern crate error_chain;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Div, Sub};

use errors::*;

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

fn main() {
    if let Err(e) = run() {
        println!("error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let file = env::args()
        .nth(1)
        .ok_or(Error::from("Expecting filename argument"))?;

    let file = File::open(file)?;
    let file = BufReader::new(file);
    let sum: u32 = file
        .lines()
        .map(|line| {
            line.expect("Could not read line")
                .parse::<u32>()
                .expect("Could not parse mass")
        })
        .map(fuel_required)
        .sum();
    println!("{}", sum);
    Ok(())
}

fn fuel_required(mass: u32) -> u32 {
    mass.div(3).sub(2)
}

#[cfg(test)]
mod tests {
    use super::fuel_required;

    #[test]
    fn mass_of_12_needs_2_fuel() {
        assert_eq!(2, fuel_required(12));
    }

    #[test]
    fn mass_of_14_needs_2_fuel() {
        assert_eq!(2, fuel_required(14));
    }

    #[test]
    fn mass_of_1969_needs_654_fuel() {
        assert_eq!(654, fuel_required(1969));
    }

    #[test]
    fn mass_of_100756_needs_33583_fuel() {
        assert_eq!(33583, fuel_required(100756));
    }
}
