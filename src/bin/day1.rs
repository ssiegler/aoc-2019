use std::io;
use std::io::BufRead;
use std::iter::successors;
use std::ops::Div;

fn main() {
    let masses = read_masses();
    let summed_fuel_requirements: u32 = masses.iter().cloned().map(fuel_required).sum();
    println!(
        "The sum of the fuel requirements is {}",
        summed_fuel_requirements
    );
    let total_fuel_required: u32 = masses.iter().cloned().map(total_fuel_required).sum();
    println!(
        "The sum of the total fuel requirements is {}",
        total_fuel_required
    );
}

fn read_masses() -> Vec<u32> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.expect("Could not read line")
                .parse::<u32>()
                .expect("Could not parse mass")
        })
        .collect()
}

fn total_fuel_required(mass: u32) -> u32 {
    successors(Some(mass), |&mass| Some(fuel_required(mass)))
        .skip(1)
        .take_while(|&fuel| fuel > 0)
        .sum()
}

fn fuel_required(mass: u32) -> u32 {
    mass.div(3).saturating_sub(2)
}

#[cfg(test)]
mod tests {
    use crate::total_fuel_required;

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
        assert_eq!(33583, fuel_required(100_756));
    }

    #[test]
    fn total_fuel_for_mass_14_is_2() {
        assert_eq!(2, total_fuel_required(14));
    }

    #[test]
    fn total_fuel_for_mass_1969_is_966() {
        assert_eq!(966, total_fuel_required(1969));
    }

    #[test]
    fn total_fuel_for_mass_100756_is_50346() {
        assert_eq!(50346, total_fuel_required(100_756));
    }
}
