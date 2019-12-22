use std::mem;

use super::file_loader;

pub fn run(part: i32) {
    let input = file_loader::load_file("7.input");
    let result = calculate_highest_thruster_signal(&input, part as u32);
    println!("Result is {}", result);
}

fn calculate_highest_thruster_signal(input: &str, part: u32) -> i32 {
    let intcode: Vec<i32> = input.split(",")
        .map(|number| number.parse::<i32>().unwrap())
        .collect();

    let low = if part == 1 {
        0
    } else {
        5
    };
    let high = if part == 1 {
        5
    } else {
        10
    };

    let sequences = build_sequences(low, high);
    
    let mut highest = 0;
    for sequence in sequences {
        let signal = signal_from_sequence(&intcode, sequence, part == 2);

        if signal > highest {
            highest = signal;
        }
    }

    return highest;
}

fn build_sequences(low: u32, high: u32) -> Vec<Vec<u32>> {
    let mut sequences = vec![];
    build_sequence(&mut sequences, vec![], low, high);
    return sequences;
}

fn build_sequence(sequences: &mut Vec<Vec<u32>>, taken: Vec<u32>, low: u32, high: u32) {
    for i in low..high {
        if taken.contains(&i) {
            continue;
        }

        let mut next = taken.to_vec();
        next.push(i);
        build_sequence(sequences, next, low, high);
    }

    if taken.len() == (high-low) as usize {
        sequences.push(taken)
    }
}

fn signal_from_sequence(intcode: &Vec<i32>, sequence: Vec<u32>, recusive: bool) -> i32 {
    let mut signal = 0;
    let mut amplifiers: Vec<IntCodeComputer> = vec![];
    let debug = false;

    for phase_setting in &sequence {
        amplifiers.push(IntCodeComputer {
            intcode: intcode.to_vec(),
            inputs: vec![*phase_setting as i32],
            outputs: vec![],
            ptr: 0,
            debug: debug,
        });
    }

    if recusive {
        let mut count = 0;
        loop {
            let index = count % amplifiers.len();
            let amp: &mut IntCodeComputer = &mut amplifiers[index];
            amp.inputs.push(signal);
            let result = amp.run();
            let output = amp.outputs.remove(0);
            if debug {
                println!("\nAmp: {}, ps {}, so {}, out: {}", index, sequence[index], signal, output);
            }
            signal = output;
            count += 1;

            if result == HALT && index == amplifiers.len() - 1 {
                break;
            }
        }
    } else {
        for (index, ps) in sequence.iter().enumerate() {
            let amp: &mut IntCodeComputer = &mut amplifiers[index];
            amp.inputs.push(signal);
            amp.run();
            let output = amp.outputs.remove(0);
            signal = output;
            if debug {
                println!("\nAmp: {}, ps {}, so {}, out: {}", index, ps, signal, output);
            }
        }
    }

    return signal;
}

pub struct IntCodeComputer {
    pub intcode: Vec<i32>,
    inputs: Vec<i32>,
    outputs: Vec<i32>,
    ptr: usize,
    debug: bool,
}

const NEED_INPUT: u32 = 2;
const HALT: u32 = 1;
const RESULT_OK: u32 = 0;
impl IntCodeComputer {

    pub fn run(&mut self) -> u32 {
        if self.debug {
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
            99 => return HALT,
            _ => panic!("Unknown opcode {}. Something went wrong", opcode)
        }

        if self.debug {
            println!("--State: ptr: {}. Intcode:", self.ptr);
            print!("--");
            for (index, item) in self.intcode.iter().enumerate() {
                print!("[{}: {}], ", index, item);
            }
            print!("\n");
        }
    
        return RESULT_OK;
    }

    fn operation_from_opcode(&self, opcode: i32) -> i32 {
        return opcode % 100;
    }

    fn add(&mut self, opcode: i32) {
        let parameter_modes = self.modes_from_opcode(opcode, 3);
        let value1: i32 = self.value_from_parameter(parameter_modes[0], &self.intcode, self.ptr + 1);
        let value2: i32 = self.value_from_parameter(parameter_modes[1], &self.intcode, self.ptr + 2);
        let result_address: usize = self.intcode[self.ptr + 3] as usize;
        let result: i32 = value1 + value2;
        if self.debug {
            println!("--Adding {} and {} to get {} to put at index {}", value1, value2, result, result_address);
        }
        mem::replace(&mut self.intcode[result_address], result);
        self.ptr += 4;
    }
    
    fn multiply(&mut self, opcode: i32) {
        let parameter_modes = self.modes_from_opcode(opcode, 3);
        let value1: i32 = self.value_from_parameter(parameter_modes[0], &self.intcode, self.ptr + 1);
        let value2: i32 = self.value_from_parameter(parameter_modes[1], &self.intcode, self.ptr + 2);
        let result_address: usize = self.intcode[self.ptr + 3] as usize;
        let result: i32 = value1 * value2;
        if self.debug {
            println!("--Multiplying {} and {} to get {} to put at index {}", value1, value2, result, result_address);
        }
        mem::replace(&mut self.intcode[result_address], result);
        self.ptr += 4;
    }
    
    fn input(&mut self, _opcode: i32) -> u32 {
        let result_address: usize = self.intcode[self.ptr + 1] as usize;
        if self.inputs.is_empty() {
            if self.debug {
                println!("--Need Input for address {}. Waiting...", result_address);
            }
            return NEED_INPUT;
        }
        let input = self.inputs.remove(0);
        if self.debug {
            println!("--Inputting {} to address {}", input, result_address);
        }
        mem::replace(&mut self.intcode[result_address], input);
        self.ptr += 2;
        return 0;
    }
    
    fn output(&mut self, opcode: i32) {
        let parameter_modes = self.modes_from_opcode(opcode, 1);
        let output = self.value_from_parameter(parameter_modes[0], &self.intcode, self.ptr + 1);
        if self.debug {
            println!("--Outputting {} from address {}", output, self.intcode[self.ptr + 1]);
        }
        self.outputs.push(output);
        self.ptr += 2;
    }
    
    fn jump_if_true(&mut self, opcode: i32) {
        let parameter_modes = self.modes_from_opcode(opcode, 2);
        let value1: i32 = self.value_from_parameter(parameter_modes[0], &self.intcode, self.ptr + 1);
        let value2: i32 = self.value_from_parameter(parameter_modes[1], &self.intcode, self.ptr + 2);
        if self.debug {
            println!("--Jump if value1 != 0: Value1: {}, Value2(jump): {}", value1, value2);
        }
        if value1 != 0 {
            self.ptr = value2 as usize;
        } else {
            self.ptr += 3;
        }
    }
    
    fn jump_if_false(&mut self, opcode: i32) {
        let parameter_modes = self.modes_from_opcode(opcode, 2);
        let value1: i32 = self.value_from_parameter(parameter_modes[0], &self.intcode, self.ptr + 1);
        let value2: i32 = self.value_from_parameter(parameter_modes[1], &self.intcode, self.ptr + 2);
        if self.debug {
            println!("--Jump if value1 == 0: Value1: {}, Value2: {}", value1, value2);
        }
        if value1 == 0 {
            self.ptr = value2 as usize;
        } else {
            self.ptr += 3;
        }
    }
    
    fn less_then(&mut self, opcode: i32) {
        let parameter_modes = self.modes_from_opcode(opcode, 3);
        let value1: i32 = self.value_from_parameter(parameter_modes[0], &self.intcode, self.ptr + 1);
        let value2: i32 = self.value_from_parameter(parameter_modes[1], &self.intcode, self.ptr + 2);
        let value3: usize = self.intcode[self.ptr + 3] as usize;
        if self.debug {
            println!("--Less than: Value1 {}, Value2: {}, Value3: {}", value1, value2, value3);
        }
        if value1 < value2 {
            mem::replace(&mut self.intcode[value3], 1);
        } else {
            mem::replace(&mut self.intcode[value3], 0);
        }
        self.ptr += 4;
    }
    
    fn equals(&mut self, opcode: i32) {
        let parameter_modes = self.modes_from_opcode(opcode, 3);
        let value1: i32 = self.value_from_parameter(parameter_modes[0], &self.intcode, self.ptr + 1);
        let value2: i32 = self.value_from_parameter(parameter_modes[1], &self.intcode, self.ptr + 2);
        let value3: usize = self.intcode[self.ptr + 3] as usize;
        if self.debug {
            println!("--Equals: Value1 {}, Value2: {}, Value 3: {}", value1, value2, value3);
        }
        if value1 == value2 {
            mem::replace(&mut self.intcode[value3], 1);
        } else {
            mem::replace(&mut self.intcode[value3], 0);
        }
        self.ptr += 4;
    }
    
    fn modes_from_opcode(&self, opcode: i32, parameters: usize) -> Vec<i32> {
        let params = opcode / 100;
        let mut results = vec![];
        let base: i32 = 10;
        for i in 0..parameters {
            let power = (i + 1) as u32;
            results.push((params % base.pow(power)) / base.pow(power-1));
        }
    
        return results;
    }
    
    fn value_from_parameter(&self, parameter: i32, intcode: &Vec<i32>, ptr: usize) -> i32 {
        return match parameter {
            0 => {
                let value_at_pointer = intcode[ptr];
                assert!(value_at_pointer >= 0);
                intcode[value_at_pointer as usize]
            },
            1 => intcode[ptr],
            _ => panic!("parameter value {} not understood", parameter)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day7_example1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let expected = 43210;

        assert_eq!(calculate_highest_thruster_signal(input, 1), expected);
    }

    #[test]
    fn test_day7_example2() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let expected = 54321;

        assert_eq!(calculate_highest_thruster_signal(input, 1), expected);
    }

    #[test]
    fn test_day7_example3() {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let expected = 65210;

        assert_eq!(calculate_highest_thruster_signal(input, 1), expected);
    }

    #[test]
    fn test_day7_example4() {
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let expected = 139629729;

        assert_eq!(calculate_highest_thruster_signal(input, 2), expected);
    }

    #[test]
    fn test_day7_example5() {
        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let expected = 18216;

        assert_eq!(calculate_highest_thruster_signal(input, 2), expected);
    }

    #[test]
    fn test_build_sequence() {
        let expected = [[0,1,2], [0,2,1], [1,0,2], [1,2,0], [2,0,1], [2,1,0]];
        let low = 0;
        let high = 3;

        let sequences = build_sequences(low, high);

        assert_eq!(sequences, expected);
    }

    #[test]
    fn test_simple_sequence() {
        let intcode = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let sequence: Vec<u32> = vec![4,3,2,1,0];
        let expected = 43210;

        assert_eq!(signal_from_sequence(&intcode, sequence, false), expected);
    }

    #[test]
    fn test_simple_recursive_sequence() {
        let intcode = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        let sequence: Vec<u32> = vec![9,8,7,6,5];
        let expected = 139629729;

        assert_eq!(signal_from_sequence(&intcode, sequence, true), expected);
    }
}