use std::io;
use std::io::BufRead;

fn main() {
    let mut program = read_program();
    restore_alarm_state(&mut program);
    run_int_code(&mut program);
    println!("Position 0 contains {}", program[0])
}

fn restore_alarm_state(program: &mut Vec<usize>) {
    program[1] = 12;
    program[2] = 2;
}

fn read_program() -> Vec<usize> {
    io::stdin()
        .lock()
        .split(b',')
        .map(|bytes| {
            let bytes = bytes.expect("IO problem");
            std::str::from_utf8(&bytes)
                .expect("Encoding problem")
                .trim()
                .parse::<usize>()
                .expect("Could not read program")
        })
        .collect()
}

fn run_int_code(program: &mut [usize]) {
    for position in (0..program.len()).step_by(4) {
        match program[position] {
            1 => {
                program[program[position + 3]] =
                    program[program[position + 1]] + program[program[position + 2]];
            }
            2 => {
                program[program[position + 3]] =
                    program[program[position + 1]] * program[program[position + 2]];
            }
            99 => {
                break;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_1_adds_arguments() {
        let mut program: Vec<usize> = vec![1, 0, 0, 0, 99];
        run_int_code(&mut program);
        assert_eq!(vec![2, 0, 0, 0, 99], program);
    }

    #[test]
    fn opcode_2_multiplies_arguments() {
        let mut program: Vec<usize> = vec![2, 3, 0, 3, 99];
        run_int_code(&mut program);
        assert_eq!(vec![2, 3, 0, 6, 99], program);
    }

    #[test]
    fn opcode_99_terminates() {
        let mut program: Vec<usize> = vec![2, 4, 4, 5, 99, 0];
        run_int_code(&mut program);
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], program);
    }

    #[test]
    fn programs_can_self_modify() {
        let mut program: Vec<usize> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run_int_code(&mut program);
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], program);
    }
}
