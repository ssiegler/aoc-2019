use aoc_2019::intcode::io::read_program;
use aoc_2019::intcode::Computer;

fn main() {
    let program = read_program().expect("Could not read program");
    let output = Computer::execute(&program, &[1]).expect("Could not execute program");
    let (code, tests) = output.split_last().expect("No outputs");
    assert!(
        tests.iter().all(|error| *error == 0),
        "Failed tests {:?}",
        tests
    );
    println!("Diagnostic code: {}", *code);
}
