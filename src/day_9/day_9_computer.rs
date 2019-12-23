
use std::mem;

const NEED_INPUT: u32 = 2;
const HALT: u32 = 1;
const RESULT_OK: u32 = 0;
const DEBUG: bool = false;

pub struct IntCodeComputer {
    pub intcode: Vec<i64>,
    inputs: Vec<i64>,
    pub outputs: Vec<i64>,
    ptr: usize,
    relative_base: i64,
}

impl IntCodeComputer {
    pub fn new(intcode: &Vec<i64>, default_inputs: Vec<i64>) -> IntCodeComputer {
        return IntCodeComputer {
            intcode: intcode.to_vec(),
            inputs: default_inputs,
            outputs: vec![],
            ptr: 0,
            relative_base: 0,
        };
    }

    pub fn run(&mut self) -> u32 {
        if DEBUG {
            println!("\nRunning intcode: {:?}", self.intcode);
        }
        loop {
            let result = self.next_operation();
            if result != RESULT_OK {
                return result;
            }
        }
    }

    fn next_operation(&mut self) -> u32 {
        let opcode = self.intcode[self.ptr];
        let operation = self.operation_from_opcode(opcode);

        if DEBUG {
            println!("\n--Opcode {}, operation {}", opcode, operation);
        }
        
        match operation {
            1 => self.add(opcode),
            2 => self.multiply(opcode),
            3 => {
                let result = self.input(opcode);
                if result == NEED_INPUT {
                    // halt until input
                    return NEED_INPUT;
                }
            },
            4 => self.output(opcode),
            5 => self.jump_if_true(opcode),
            6 => self.jump_if_false(opcode),
            7 => self.less_then(opcode),
            8 => self.equals(opcode),
            9 => self.change_relative_base(opcode),
            99 => return HALT,
            _ => panic!("Unknown opcode {}. Something went wrong", opcode)
        }

        if DEBUG {
            println!("--State: ptr: {}. Intcode:", self.ptr);
            print!("--");
            for (index, item) in self.intcode.iter().enumerate() {
                print!("[{}: {}], ", index, item);
            }
            print!("\n");
        }
    
        return RESULT_OK;
    }

    fn operation_from_opcode(&self, opcode: i64) -> i64 {
        return opcode % 100;
    }

    fn add(&mut self, opcode: i64) {
        let parameter_modes = self.modes_from_opcode(opcode, 3);
        let value1: i64 = self.value_from_parameter(parameter_modes[0], self.ptr + 1);
        let value2: i64 = self.value_from_parameter(parameter_modes[1], self.ptr + 2);
        let result: i64 = value1 + value2;
        if DEBUG {
            println!("--Adding {} and {} to get {}", value1, value2, result);
        }
        
        self.update_value_from_parameter(parameter_modes[2], self.ptr + 3, result);
        self.ptr += 4;
    }
    
    fn multiply(&mut self, opcode: i64) {
        let parameter_modes = self.modes_from_opcode(opcode, 3);
        let value1: i64 = self.value_from_parameter(parameter_modes[0], self.ptr + 1);
        let value2: i64 = self.value_from_parameter(parameter_modes[1], self.ptr + 2);
        let result: i64 = value1 * value2;
        if DEBUG {
            println!("--Multiplying {} and {} to get {}", value1, value2, result);
        }
        
        self.update_value_from_parameter(parameter_modes[2], self.ptr + 3, result);
        self.ptr += 4;
    }
    
    fn input(&mut self, opcode: i64) -> u32 {
        let parameter_modes = self.modes_from_opcode(opcode, 1);
        if self.inputs.is_empty() {
            if DEBUG {
                println!("--Need Input. Waiting...");
            }
            return NEED_INPUT;
        }
        let input = self.inputs.remove(0);
        if DEBUG {
            println!("--Inputting {}", input);
        }
        
        self.update_value_from_parameter(parameter_modes[0], self.ptr + 1, input);
        self.ptr += 2;
        return RESULT_OK;
    }
    
    fn output(&mut self, opcode: i64) {
        let parameter_modes = self.modes_from_opcode(opcode, 1);
        let output = self.value_from_parameter(parameter_modes[0], self.ptr + 1);
        if DEBUG {
            println!("--Outputting {} from address {}", output, self.intcode[self.ptr + 1]);
        }
        self.outputs.push(output);
        self.ptr += 2;
    }
    
    fn jump_if_true(&mut self, opcode: i64) {
        let parameter_modes = self.modes_from_opcode(opcode, 2);
        let value1: i64 = self.value_from_parameter(parameter_modes[0], self.ptr + 1);
        let value2: i64 = self.value_from_parameter(parameter_modes[1], self.ptr + 2);
        if DEBUG {
            println!("--JumpIfTrue: if {} != 0, Jump to {}", value1, value2);
        }
        if value1 != 0 {
            self.ptr = value2 as usize;
        } else {
            self.ptr += 3;
        }
    }
    
    fn jump_if_false(&mut self, opcode: i64) {
        let parameter_modes = self.modes_from_opcode(opcode, 2);
        let value1: i64 = self.value_from_parameter(parameter_modes[0], self.ptr + 1);
        let value2: i64 = self.value_from_parameter(parameter_modes[1], self.ptr + 2);
        if DEBUG {
            println!("--JumpIfFalse: If {} == 0, Jump to {}", value1, value2);
        }
        if value1 == 0 {
            self.ptr = value2 as usize;
        } else {
            self.ptr += 3;
        }
    }
    
    fn less_then(&mut self, opcode: i64) {
        let parameter_modes = self.modes_from_opcode(opcode, 3);
        let value1: i64 = self.value_from_parameter(parameter_modes[0], self.ptr + 1);
        let value2: i64 = self.value_from_parameter(parameter_modes[1], self.ptr + 2);
        if DEBUG {
            println!("--Less than: If {} is less than {} 1 else 0", value1, value2);
        }

        if value1 < value2 {
            self.update_value_from_parameter(parameter_modes[2], self.ptr + 3, 1);
        } else {
            self.update_value_from_parameter(parameter_modes[2], self.ptr + 3, 0);
        }
        self.ptr += 4;
    }
    
    fn equals(&mut self, opcode: i64) {
        let parameter_modes = self.modes_from_opcode(opcode, 3);
        let value1: i64 = self.value_from_parameter(parameter_modes[0], self.ptr + 1);
        let value2: i64 = self.value_from_parameter(parameter_modes[1], self.ptr + 2);
        if DEBUG {
            println!("--Equals: If {} is equal to {} then 1 else 0", value1, value2);
        }

        if value1 == value2 {
            self.update_value_from_parameter(parameter_modes[2], self.ptr + 3, 1);
        } else {
            self.update_value_from_parameter(parameter_modes[2], self.ptr + 3, 0);
        }
        self.ptr += 4;
    }

    fn change_relative_base(&mut self, opcode: i64) {
        let parameter_modes = self.modes_from_opcode(opcode, 1);
        let value1: i64 = self.value_from_parameter(parameter_modes[0], self.ptr + 1);
        self.relative_base += value1;
        if DEBUG {
            println!("--Change relative base by {} to {}", value1, self.relative_base);
        }
        self.ptr += 2;
    }
    
    fn modes_from_opcode(&self, opcode: i64, parameters: usize) -> Vec<i64> {
        let params = opcode / 100;
        let mut results = vec![];
        let base: i64 = 10;
        for i in 0..parameters {
            let power = (i + 1) as u32;
            results.push((params % base.pow(power)) / base.pow(power-1));
        }
    
        return results;
    }
    
    fn value_from_parameter(&mut self, parameter: i64, ptr: usize) -> i64 {
        return match parameter {
            0 => {
                let address_at_pointer = self.intcode[ptr];
                assert!(address_at_pointer >= 0);

                self.get_value(address_at_pointer as usize)
            },
            1 => self.get_value(ptr),
            2 => {
                let address_at_pointer = self.intcode[ptr];
                let relative = address_at_pointer + self.relative_base;
                assert!(relative >= 0);

                self.get_value(relative as usize)
            }
            _ => panic!("parameter value {} not understood", parameter)
        };
    }

    fn update_value_from_parameter(&mut self, parameter: i64, ptr: usize, value: i64) {
        match parameter {
            0 | 1 => {
                let address_at_pointer = self.intcode[ptr];
                assert!(address_at_pointer >= 0);

                self.replace_value(address_at_pointer as usize, value)
            },
            2 => {
                let address_at_pointer = self.intcode[ptr];
                let relative = address_at_pointer + self.relative_base;
                assert!(relative >= 0);
                
                self.replace_value(relative as usize, value);
            }
            _ => panic!("parameter value {} not understood", parameter)
        }
    }

    fn replace_value(&mut self, address: usize, value: i64) {
        let length = self.intcode.len();
        if address >= length {
            self.intcode.resize(address + 1, 0);
        }
        if DEBUG {
            println!("--Update address {} to value {}", address, value);
        }
        mem::replace(&mut self.intcode[address], value);
    }

    fn get_value(&mut self, address: usize) -> i64 {
        let length = self.intcode.len();
        if address >= length {
            self.intcode.resize(address + 1, 0);
        }
        return self.intcode[address];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day9_example1() {
        let intcode = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let expected = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];

        let mut computer = IntCodeComputer::new(&intcode, vec![]);
        computer.run();

        assert_eq!(computer.outputs, expected);
    }

    #[test]
    fn test_day9_example2() {
        let intcode = vec![1102,34915192,34915192,7,4,7,99,0];
        let expected = 16;

        let mut computer = IntCodeComputer::new(&intcode, vec![]);
        computer.run();

        assert_eq!(format!("{}", computer.outputs[0]).len(), expected);
    }

    #[test]
    fn test_day9_example3() {
        let intcode = vec![104,1125899906842624,99];
        let expected = 1125899906842624;

        let mut computer = IntCodeComputer::new(&intcode, vec![]);
        computer.run();

        assert_eq!(computer.outputs[0], expected);
    }

    #[test]
    fn test_day9_203() {
        let intcode = vec![9, 3, 203, 6, 4, 12, 99];
        let inputs = vec![1];
        let expected = 1;

        let mut computer = IntCodeComputer::new(&intcode, inputs);
        computer.run();

        assert_eq!(computer.outputs[0], expected);
    }


    // OLD IntCode Computer tests
    #[test]
    fn test_run_intcode_example1() {
        let intcode: Vec<i64> = vec![1,0,0,0,99];
        let end = vec![2,0,0,0,99];
        let mut computer = IntCodeComputer::new(&intcode, vec![]);
        computer.run();

        assert_eq!(computer.intcode, end);
    }

    #[test]
    fn test_run_intcode_example2() {
        let intcode: Vec<i64> = vec![2,3,0,3,99];
        let end = vec![2,3,0,6,99];
        let mut computer = IntCodeComputer::new(&intcode, vec![]);
        computer.run();
        
        assert_eq!(computer.intcode, end);
    }

    #[test]
    fn test_run_intcode_example3() {
        let intcode: Vec<i64> = vec![2,4,4,5,99,0];
        let end = vec![2,4,4,5,99,9801];
        let mut computer = IntCodeComputer::new(&intcode, vec![]);
        computer.run();
        
        assert_eq!(computer.intcode, end);
    }

    #[test]
    fn test_run_intcode_example4() {
        let intcode: Vec<i64> = vec![1,1,1,4,99,5,6,0,99];
        let end = vec![30,1,1,4,2,5,6,0,99];
        let mut computer = IntCodeComputer::new(&intcode, vec![]);
        computer.run();
        
        assert_eq!(computer.intcode, end);
    }

    #[test]
    fn test_add() {
        let intcode: Vec<i64> = vec![1001,4,3,4,33];
        let expected = vec![1001,4,3,4,36];

        let mut computer = IntCodeComputer::new(&intcode, vec![]);

        computer.next_operation();

        assert_eq!(computer.intcode, expected);
        assert_eq!(computer.ptr, 4);
    }

    #[test]
    fn test_multiply() {
        let intcode: Vec<i64> = vec![1002,4,3,4,33];
        let expected = vec![1002,4,3,4,99];

        let mut computer = IntCodeComputer::new(&intcode, vec![]);

        computer.next_operation();

        assert_eq!(computer.intcode, expected);
        assert_eq!(computer.ptr, 4);
    }

    #[test]
    fn test_input() {
        let intcode: Vec<i64> = vec![3,5,0,0,0,0];
        let expected = vec![3,5,0,0,0,44];

        let mut computer = IntCodeComputer::new(&intcode, vec![44]);

        computer.next_operation();

        assert_eq!(computer.intcode, expected);
        assert_eq!(computer.ptr, 2);
    }

    #[test]
    fn test_output() {
        let intcode: Vec<i64> = vec![4,5,0,0,0,33];
        let expected = vec![4,5,0,0,0,33];

        let mut computer = IntCodeComputer::new(&intcode, vec![]);

        computer.next_operation();

        assert_eq!(computer.intcode, expected);
        assert_eq!(computer.ptr, 2);
    }

    #[test]
    fn test_jump_if_true_mode_position_is_true() {
        let intcode: Vec<i64> = vec![5,3,6,2,0,33,12];
        let expected = 12;

        let mut computer = IntCodeComputer::new(&intcode, vec![]);

        computer.next_operation();

        assert_eq!(computer.ptr, expected);
    }

    #[test]
    fn test_jump_if_true_mode_immidiate_is_true() {
        let intcode: Vec<i64> = vec![1105,1,6,0,0,33,12];
        let expected = 6;

        let mut computer = IntCodeComputer::new(&intcode, vec![]);

        computer.next_operation();

        assert_eq!(computer.ptr, expected);
    }

    #[test]
    fn test_jump_if_false_mode_position_is_false() {
        let intcode: Vec<i64> = vec![6,3,6,0,0,33,12];
        let expected = 12;

        let mut computer = IntCodeComputer::new(&intcode, vec![]);

        computer.next_operation();

        assert_eq!(computer.ptr, expected);
    }

    #[test]
    fn test_jump_if_false_mode_immidiate_is_false() {
        let intcode: Vec<i64> = vec![1106,0,6,0,0,33];
        let expected = 6;

        let mut computer = IntCodeComputer::new(&intcode, vec![]);

        computer.next_operation();

        assert_eq!(computer.ptr, expected);
    }

    #[test]
    fn test_less_than_mode_position() {
        let intcode: Vec<i64> = vec![7,4,5,6,6,7,7,24];
        let expected = vec![7,4,5,6,6,7,1,24];
        let expected_ptr = 4;

        let mut computer = IntCodeComputer::new(&intcode, vec![]);

        computer.next_operation();

        assert_eq!(computer.intcode, expected);
        assert_eq!(computer.ptr, expected_ptr);
    }

    #[test]
    fn test_less_than_mode_immidiate() {
        let intcode: Vec<i64> = vec![11107,3,6,5,5,33];
        let expected = vec![11107,3,6,5,5,1];
        let expected_ptr = 4;

        let mut computer = IntCodeComputer::new(&intcode, vec![]);

        computer.next_operation();

        assert_eq!(computer.intcode, expected);
        assert_eq!(computer.ptr, expected_ptr);
    }
}