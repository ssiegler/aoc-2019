use std::convert::TryFrom;

#[derive(Default)]
pub struct Computer {
    instruction_pointer: usize,
    memory: Vec<i32>,
    output: Vec<i32>,
    input: Vec<i32>,
}

impl Computer {
    pub fn load(&mut self, program: &[i32]) {
        self.instruction_pointer = 0;
        self.memory = program.to_vec();
    }

    pub fn restore_alarm_state(&mut self) {
        self.memory[1] = 12;
        self.memory[2] = 2;
    }

    pub fn set_inputs(&mut self, noun: i32, verb: i32) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    pub fn execute_program(&mut self) {
        while self.load_opcode() != 99 {
            self.execute_operation();
        }
    }

    pub fn get_output(&self) -> i32 {
        self.memory[0]
    }

    fn load_opcode(&self) -> usize {
        usize::try_from(self.memory[self.instruction_pointer]).expect("Invalid opcode")
    }

    fn load_argument(&self, index: usize) -> i32 {
        self.memory[self.instruction_pointer + index]
    }

    fn execute_operation(&mut self) {
        match self.load_opcode() {
            1 => {
                let result_address =
                    usize::try_from(self.load_argument(3)).expect("Invalid address");
                let operand1_address =
                    usize::try_from(self.load_argument(1)).expect("Invalid address");
                let operand2_address =
                    usize::try_from(self.load_argument(2)).expect("Invalid address");
                self.memory[result_address] =
                    self.memory[operand1_address] + self.memory[operand2_address];
                self.instruction_pointer += 4;
            }
            2 => {
                let result_address =
                    usize::try_from(self.load_argument(3)).expect("Invalid address");
                let operand1_address =
                    usize::try_from(self.load_argument(1)).expect("Invalid address");
                let operand2_address =
                    usize::try_from(self.load_argument(2)).expect("Invalid address");
                self.memory[result_address] =
                    self.memory[operand1_address] * self.memory[operand2_address];
                self.instruction_pointer += 4;
            }
            3 => {
                let operand1_address =
                    usize::try_from(self.load_argument(1)).expect("Invalid address");
                self.memory[operand1_address] = self.input.pop().expect("Missing input");
                self.instruction_pointer += 2;
            }
            4 => {
                let operand1_address =
                    usize::try_from(self.load_argument(1)).expect("Invalid address");
                self.output.push(self.memory[operand1_address]);
                self.instruction_pointer += 2;
            }
            _ => {
                unimplemented!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_1_adds_arguments() {
        let mut computer = Computer::default();
        computer.load(&[1, 0, 0, 0, 99]);
        computer.execute_program();
        assert_eq!(computer.memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn opcode_2_multiplies_arguments() {
        let mut computer = Computer::default();
        computer.load(&[2, 3, 0, 3, 99]);
        computer.execute_program();
        assert_eq!(computer.memory, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn opcode_99_terminates() {
        let mut computer = Computer::default();
        computer.load(&[2, 4, 4, 5, 99, 0]);
        computer.execute_program();
        assert_eq!(computer.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn programs_can_self_modify() {
        let mut computer = Computer::default();
        computer.load(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
        computer.execute_program();
        assert_eq!(computer.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn supports_negative_numbers() {
        let mut computer = Computer::default();
        computer.load(&[1, 5, 6, 0, 99, 5, -6]);
        computer.execute_program();
        assert_eq!(computer.memory, vec![-1, 5, 6, 0, 99, 5, -6]);
    }

    #[test]
    fn supports_output() {
        let mut computer = Computer::default();
        computer.load(&[4, 3, 99, 23]);
        computer.execute_program();
        assert_eq!(computer.output, &[23]);
    }

    #[test]
    fn supports_input() {
        let mut computer = Computer::default();
        computer.load(&[3, 0, 99]);
        computer.input.push(13);
        computer.execute_program();
        assert_eq!(computer.memory, &[13, 0, 99]);
    }

    #[test]
    fn can_echo() {
        let mut computer = Computer::default();
        computer.load(&[3, 0, 4, 0, 99]);
        computer.input.push(13);
        computer.execute_program();
        assert_eq!(computer.memory, &[13, 0, 4, 0, 99]);
        assert_eq!(computer.output, &[13]);
    }
}
