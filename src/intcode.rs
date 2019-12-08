pub struct Computer {
    instruction_pointer: usize,
    memory: Vec<usize>,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            instruction_pointer: 0,
            memory: vec![99],
        }
    }

    pub fn load(&mut self, program: Vec<usize>) {
        self.instruction_pointer = 0;
        self.memory = program;
    }

    pub fn restore_alarm_state(&mut self) {
        self.memory[1] = 12;
        self.memory[2] = 2;
    }

    pub fn execute_program(&mut self) {
        while self.load_opcode() != 99 {
            self.execute_operation();
        }
    }

    pub fn get_output(&self) -> usize {
        self.memory[0]
    }

    fn load_opcode(&self) -> usize {
        self.memory[self.instruction_pointer]
    }

    fn load_argument(&self, index: usize) -> usize {
        self.memory[self.instruction_pointer + index]
    }

    fn execute_operation(&mut self) {
        match self.load_opcode() {
            1 => {
                let result_address = self.load_argument(3);
                self.memory[result_address] =
                    self.memory[self.load_argument(1)] + self.memory[self.load_argument(2)];
                self.instruction_pointer += 4;
            }
            2 => {
                let result_address = self.load_argument(3);
                self.memory[result_address] =
                    self.memory[self.load_argument(1)] * self.memory[self.load_argument(2)];
                self.instruction_pointer += 4;
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
        let mut computer = Computer::new();
        computer.load(vec![1, 0, 0, 0, 99]);
        computer.execute_program();
        assert_eq!(vec![2, 0, 0, 0, 99], computer.memory);
    }

    #[test]
    fn opcode_2_multiplies_arguments() {
        let mut computer = Computer::new();
        computer.load(vec![2, 3, 0, 3, 99]);
        computer.execute_program();
        assert_eq!(vec![2, 3, 0, 6, 99], computer.memory);
    }

    #[test]
    fn opcode_99_terminates() {
        let mut computer = Computer::new();
        computer.load(vec![2, 4, 4, 5, 99, 0]);
        computer.execute_program();
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], computer.memory);
    }

    #[test]
    fn programs_can_self_modify() {
        let mut computer = Computer::new();
        computer.load(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        computer.execute_program();
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], computer.memory);
    }
}
