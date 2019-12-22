#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Parameter {
    Address { address: usize },
    Value { value: i32 },
}

impl Parameter {
    fn address(address: usize) -> Self {
        Parameter::Address { address }
    }

    fn value(value: i32) -> Self {
        Parameter::Value { value }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    pub opcode: usize,
    pub parameters: Vec<Parameter>,
    pub length: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidOpcode { opcode: usize },
    UnknownParameterMode { mode: usize },
}

impl Instruction {
    pub fn decode(program: &[i32]) -> Result<Self, Error> {
        let opcode = program[0] as usize;
        let parameter_modes = opcode / 100;
        let opcode = opcode % 100;
        let parameters = match opcode {
            1 | 2 => decode_parameters(&program[1..4], parameter_modes),
            3 | 4 => decode_parameters(&program[1..2], parameter_modes),
            99 => Ok(vec![]),
            _ => Err(Error::InvalidOpcode { opcode }),
        }?;
        let length = 1 + parameters.len();
        Ok(Instruction {
            opcode,
            parameters,
            length,
        })
    }
}

fn decode_parameters(parameters: &[i32], modes: usize) -> Result<Vec<Parameter>, Error> {
    parameters
        .iter()
        .scan(modes, |modes, parameter| {
            let item = decode_parameter(*parameter, *modes % 10);
            *modes /= 10;
            Some(item)
        })
        .collect()
}

fn decode_parameter(parameter: i32, mode: usize) -> Result<Parameter, Error> {
    match mode {
        0 => Ok(Parameter::address(parameter as usize)),
        1 => Ok(Parameter::value(parameter)),
        _ => Err(Error::UnknownParameterMode { mode }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_terminate() {
        assert_eq!(
            Instruction::decode(&[99]),
            Ok(Instruction {
                opcode: 99,
                parameters: vec![],
                length: 1,
            })
        );
    }

    #[test]
    fn decode_ignores_trail() {
        assert_eq!(
            Instruction::decode(&[99, 1, -1, 0]),
            Ok(Instruction {
                opcode: 99,
                parameters: vec![],
                length: 1,
            })
        );
    }

    #[test]
    fn decode_add_with_addresses() {
        assert_eq!(
            Instruction::decode(&[1, 0, 0, 0, 99]),
            Ok(Instruction {
                opcode: 1,
                parameters: vec![Parameter::address(0); 3],
                length: 4,
            })
        );
    }

    #[test]
    fn decode_input_with_value() {
        assert_eq!(
            Instruction::decode(&[1103, 42, 2, 1, 0]),
            Ok(Instruction {
                opcode: 3,
                parameters: vec![Parameter::value(42)],
                length: 2,
            })
        );
    }

    #[test]
    fn decode_output_with_address() {
        assert_eq!(
            Instruction::decode(&[4, 42, 2, 1, 0]),
            Ok(Instruction {
                opcode: 4,
                parameters: vec![Parameter::address(42)],
                length: 2,
            })
        );
    }

    #[test]
    fn decode_multiply_with_value_and_address() {
        assert_eq!(
            Instruction::decode(&[102, 33, 3, 4, 0]),
            Ok(Instruction {
                opcode: 2,
                parameters: vec![
                    Parameter::value(33),
                    Parameter::address(3),
                    Parameter::address(4),
                ],
                length: 4,
            })
        );
    }
}
