use aoc_2019::intcode::io::read_program;
use aoc_2019::intcode::{Computer, Error};

fn main() {
    let program = read_program().expect("Could not read program");
    let output = find_max_thruster_output(&program).expect("No output");
    println!("Maximal thruster output: {}", output)
}

fn compute_thruster_output(program: &[i32], phase_settings: &[i32; 5]) -> Result<i32, Error> {
    let mut input = 0;
    for phase_setting in phase_settings.iter() {
        input = Computer::execute(program, &[*phase_setting, input])?[0];
    }
    Ok(input)
}

fn find_max_thruster_output(program: &[i32]) -> Option<i32> {
    phase_settings()
        .iter()
        .flat_map(|phase_settings| compute_thruster_output(program, phase_settings))
        .max()
}

fn phase_settings() -> Vec<[i32; 5]> {
    let mut phase_settings = Vec::with_capacity(120);
    for i in 0..5 {
        for j in (0..5).filter(|&j| j != i) {
            for k in (0..5).filter(|&k| k != j && k != i) {
                for l in (0..5).filter(|&l| l != k && l != j && l != i) {
                    for m in (0..5).filter(|&m| m != l && m != k && m != j && m != i) {
                        phase_settings.push([i, j, k, l, m])
                    }
                }
            }
        }
    }
    phase_settings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_example_digits() {
        assert_eq!(
            compute_thruster_output(
                &[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                &[4, 3, 2, 1, 0]
            ),
            Ok(43210)
        );
    }

    #[test]
    fn example_digits() {
        assert_eq!(
            find_max_thruster_output(&[
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ]),
            Some(43210)
        );
    }
    #[test]
    fn example_count_down() {
        assert_eq!(
            find_max_thruster_output(&[
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ]),
            Some(54321)
        );
    }
}
