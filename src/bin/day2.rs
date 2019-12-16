use std::io;
use std::io::BufRead;
use std::iter;

use aoc_2019::intcode::Computer;

fn main() {
    let program = read_program();
    let mut computer = Computer::default();
    computer.load(&program);
    computer.restore_alarm_state();
    computer.execute_program();
    println!("Program alarm output: {}", computer.get_output());
    let (noun, verb) = (0..=99)
        .flat_map(|noun| iter::repeat(noun).zip(0..99))
        .find(|(noun, verb)| {
            computer.load(&program);
            computer.set_inputs(*noun, *verb);
            computer.execute_program();
            computer.get_output() == 19_690_720
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
