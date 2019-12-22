use super::decode;
use super::decode::{Instruction, Parameter};

#[derive(Default)]
pub struct Computer {
    memory: Vec<i32>,
    output: Vec<i32>,
    input: Vec<i32>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    InstructionError { error: decode::Error },
    UnsupportedOperation { opcode: usize },
    NoAddress { parameter: Parameter },
    InvalidAddress { address: usize },
    ParameterCount { expected: usize, got: usize },
    MissingInput,
}

impl Error {
    fn no_address(parameter: Parameter) -> Self {
        Error::NoAddress { parameter }
    }
}

impl From<decode::Error> for Error {
    fn from(error: decode::Error) -> Self {
        Error::InstructionError { error }
    }
}

impl Computer {
    pub fn execute(program: &[i32], input: &[i32]) -> Result<Vec<i32>, Error> {
        let mut computer = Computer::default();
        computer.load_program(program);
        computer.load_input(input);
        computer.execute_program()?;
        Ok(computer.output)
    }

    pub fn execute_with_memory_io(program: &[i32], noun: i32, verb: i32) -> Result<i32, Error> {
        let mut computer = Computer::default();
        computer.load_program(program);
        computer.memory[1] = noun;
        computer.memory[2] = verb;
        computer.execute_program()?;
        Ok(computer.memory[0])
    }

    fn load_program(&mut self, program: &[i32]) {
        self.memory = program.to_vec();
    }

    fn load_input(&mut self, input: &[i32]) {
        self.input = input.to_vec();
    }

    fn execute_program(&mut self) -> Result<(), Error> {
        let mut instruction_pointer = 0;
        loop {
            let Instruction {
                opcode,
                parameters,
                length,
            } = Instruction::decode(&self.memory[instruction_pointer..])?;
            instruction_pointer += length;
            match opcode {
                1 => self.add(&parameters)?,
                2 => self.multiply(&parameters)?,
                3 => self.input(&parameters)?,
                4 => self.output(&parameters)?,
                5 => self.jump(&mut instruction_pointer, &parameters, |value| value != 0)?,
                6 => self.jump(&mut instruction_pointer, &parameters, |value| value == 0)?,
                7 => self.less_than(&parameters)?,
                8 => self.equals(&parameters)?,
                99 => return Ok(()),
                _ => return Err(Error::UnsupportedOperation { opcode }),
            }
        }
    }

    fn jump(
        &self,
        instruction_pointer: &mut usize,
        parameters: &[Parameter],
        predicate: fn(i32) -> bool,
    ) -> Result<(), Error> {
        if predicate(self.load(parameters[0])?) {
            *instruction_pointer = self.load(parameters[1])? as usize
        }
        Ok(())
    }

    fn store(&mut self, parameter: Parameter, value: i32) -> Result<(), Error> {
        if let Parameter::Address { address } = parameter {
            self.memory[address] = value;
            Ok(())
        } else {
            Err(Error::no_address(parameter))
        }
    }

    fn load(&self, parameter: Parameter) -> Result<i32, Error> {
        match parameter {
            Parameter::Address { address } => self
                .memory
                .get(address)
                .cloned()
                .ok_or(Error::InvalidAddress { address }),
            Parameter::Value { value } => Ok(value),
        }
    }

    fn binary_operation(
        &mut self,
        parameters: &[Parameter],
        operation: fn(i32, i32) -> i32,
    ) -> Result<(), Error> {
        if parameters.len() == 3 {
            self.store(
                parameters[2],
                operation(self.load(parameters[0])?, self.load(parameters[1])?),
            )
        } else {
            Err(Error::ParameterCount {
                expected: 3,
                got: parameters.len(),
            })
        }
    }

    fn add(&mut self, parameters: &[Parameter]) -> Result<(), Error> {
        self.binary_operation(parameters, |a, b| a + b)
    }

    fn multiply(&mut self, parameters: &[Parameter]) -> Result<(), Error> {
        self.binary_operation(parameters, |a, b| a * b)
    }

    fn equals(&mut self, parameters: &[Parameter]) -> Result<(), Error> {
        self.binary_operation(parameters, |a, b| if a == b { 1 } else { 0 })
    }

    fn less_than(&mut self, parameters: &[Parameter]) -> Result<(), Error> {
        self.binary_operation(parameters, |a, b| if a < b { 1 } else { 0 })
    }

    fn input(&mut self, parameters: &[Parameter]) -> Result<(), Error> {
        if parameters.len() == 1 {
            let input = self.input.pop().ok_or(Error::MissingInput)?;
            self.store(parameters[0], input)
        } else {
            Err(Error::ParameterCount {
                expected: 1,
                got: parameters.len(),
            })
        }
    }

    fn output(&mut self, parameters: &[Parameter]) -> Result<(), Error> {
        if parameters.len() == 1 {
            self.output.push(self.load(parameters[0])?);
            Ok(())
        } else {
            Err(Error::ParameterCount {
                expected: 1,
                got: parameters.len(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_1_adds_arguments() {
        let mut computer = Computer::default();
        computer.load_program(&[1, 0, 0, 0, 99]);
        computer.execute_program().expect("Execution failed");
        assert_eq!(computer.memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn opcode_2_multiplies_arguments() {
        let mut computer = Computer::default();
        computer.load_program(&[2, 3, 0, 3, 99]);
        computer.execute_program().expect("Execution failed");
        assert_eq!(computer.memory, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn opcode_99_terminates() {
        let mut computer = Computer::default();
        computer.load_program(&[2, 4, 4, 5, 99, 0]);
        computer.execute_program().expect("Execution failed");
        assert_eq!(computer.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn programs_can_self_modify() {
        let mut computer = Computer::default();
        computer.load_program(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
        computer.execute_program().expect("Execution failed");
        assert_eq!(computer.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn supports_negative_numbers() {
        let mut computer = Computer::default();
        computer.load_program(&[1, 5, 6, 0, 99, 5, -6]);
        computer.execute_program().expect("Execution failed");
        assert_eq!(computer.memory, vec![-1, 5, 6, 0, 99, 5, -6]);
    }

    #[test]
    fn supports_output() {
        let mut computer = Computer::default();
        computer.load_program(&[4, 3, 99, 23]);
        computer.execute_program().expect("Execution failed");
        assert_eq!(computer.output, &[23]);
    }

    #[test]
    fn supports_input() {
        let mut computer = Computer::default();
        computer.load_program(&[3, 0, 99]);
        computer.input.push(13);
        computer.execute_program().expect("Execution failed");
        assert_eq!(computer.memory, &[13, 0, 99]);
    }

    #[test]
    fn can_echo() {
        let mut computer = Computer::default();
        computer.load_program(&[3, 0, 4, 0, 99]);
        computer.input.push(13);
        computer.execute_program().expect("Execution failed");
        assert_eq!(computer.memory, &[13, 0, 4, 0, 99]);
        assert_eq!(computer.output, &[13]);
    }

    #[test]
    fn can_compare_equality() {
        assert_eq!(
            Computer::execute(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[8]),
            Ok(vec![1])
        );
        assert_eq!(
            Computer::execute(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], &[8]),
            Ok(vec![1])
        );
    }

    #[test]
    fn can_compare_order() {
        assert_eq!(
            Computer::execute(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &[8]),
            Ok(vec![0])
        );
        assert_eq!(
            Computer::execute(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], &[8]),
            Ok(vec![0])
        );
    }

    #[test]
    fn can_jump() {
        assert_eq!(
            Computer::execute(
                &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                &[2]
            ),
            Ok(vec![1])
        );
    }
}
