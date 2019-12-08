use std::io;
use std::io::BufRead;

use aoc_2019::intcode::Computer;

fn main() {
    let program = read_program();
    let mut computer = Computer::new();
    computer.load(program);
    computer.restore_alarm_state();
    computer.execute_program();
    println!("Program output: {}", computer.get_output())
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
