use aoc_2019::intcode::io::read_program;
use aoc_2019::intcode::Computer;

fn main() {
    let program = read_program().expect("Could not read program");
    let output = Computer::execute(&program, &[1]).expect("Could not execute system 1");
    let (code, tests) = output.split_last().expect("No outputs");
    assert!(
        tests.iter().all(|error| *error == 0),
        "Failed tests {:?}",
        tests
    );
    println!("Diagnostic code for system 1: {}", *code);
    let output = Computer::execute(&program, &[5]).expect("Could not execute system 5");
    let (code, tests) = output.split_last().expect("No outputs");
    assert!(tests.is_empty(), "Got errors: {:?}", tests);
    println!("Diagnostic code for system 5: {}", *code);
}
