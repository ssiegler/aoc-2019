use std::io;
use std::io::BufRead;
use std::ops::{Div, Sub};

fn main() {
    let masses: Vec<u32> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.expect("Could not read line")
                .parse::<u32>()
                .expect("Could not parse mass")
        })
        .collect();
    let summed_fuel_requirements: u32 = masses.iter().cloned().map(fuel_required).sum();
    println!(
        "The sum of the fuel requirements is {}",
        summed_fuel_requirements
    )
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
