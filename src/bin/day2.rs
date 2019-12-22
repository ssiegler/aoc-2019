use std::io;
use std::io::BufRead;
use std::iter;

use aoc_2019::intcode::Computer;

fn main() {
    let program = read_program();
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

fn read_program() -> Vec<i32> {
    io::stdin()
        .lock()
        .split(b',')
        .map(|bytes| {
            let bytes = bytes.expect("IO problem");
            std::str::from_utf8(&bytes)
                .expect("Encoding problem")
                .trim()
                .parse::<i32>()
                .expect("Could not read program")
        })
        .collect()
}
