use std::iter;

use aoc_2019::intcode::{io::read_program, Computer};

fn main() {
    let program = read_program().expect("Could not read program");
    println!(
        "Program alarm output: {}",
        Computer::execute_with_memory_io(&program, 12, 2).expect("Execution failed")
    );
    let (noun, verb) = (0..=99)
        .flat_map(|noun| iter::repeat(noun).zip(0..99))
        .find(|(noun, verb)| {
            Computer::execute_with_memory_io(&program, *noun, *verb).expect("Execution failed")
                == 19_690_720
        })
        .expect("No solution found");
    println!("Found inputs: {}", 100 * noun + verb);
}
