use std::io;
use std::mem;

use super::file_loader;

pub fn run(part: i32) {
    let input = file_loader::load_file("5.input");

    let input_fn = || {
        println!("Your input is required. Please neter number: ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("Input was: '{}'", input.trim());
            }
            Err(error) => panic!(error),
        }

        return input.trim().parse::<i32>().unwrap();
    };
    let output_fn = |x| println!("\nOutput: {}", x);

    let result = day_five(&input, part as u32, input_fn, output_fn);

    println!("Result is {}", result)
}

fn day_five(input: &str, _part: u32, input_fn: fn() -> i32, output_fn: fn(i32) -> ()) -> i32 {
    let intcode: Vec<i32> = input.split(",")
        .map(|number| number.parse::<i32>().unwrap())
        .collect();
    
    run_intcode(intcode.to_vec(), input_fn, output_fn);
    return 0;
}

fn run_intcode(mut intcode: Vec<i32>, input_fn: fn() -> i32, output_fn: fn(i32) -> ()) -> Vec<i32> {
    let mut ptr: usize = 0;
    loop {
        let result = next_operation(&mut intcode, &mut ptr, input_fn, output_fn);
        //println!("ptr: {}", ptr);
        if result == 0 {
            break;
        }
    }

    return intcode;
}

fn next_operation(intcode: &mut Vec<i32>, ptr: &mut usize, input_fn: fn() -> i32, output_fn: fn(i32) -> ()) -> i32 {
    let opcode = intcode[*ptr];
    let operation = operation_from_opcode(opcode);
    
    match operation {
        1 => add(intcode, ptr, opcode),
        2 => multiply(intcode, ptr, opcode),
        3 => input(intcode, ptr, opcode, input_fn),
        4 => output(intcode, ptr, opcode, output_fn),
        5 => jump_if_true(intcode, ptr, opcode),
        6 => jump_if_false(intcode, ptr, opcode),
        7 => less_then(intcode, ptr, opcode),
        8 => equals(intcode, ptr, opcode),
        99 => return 0,
        _ => panic!("Unknown opcode {}. Something went wrong", opcode)
    }

    return 1;
}

fn operation_from_opcode(opcode: i32) -> i32 {
    return opcode % 100;
}

fn add(intcode: &mut Vec<i32>, ptr: &mut usize, opcode: i32) {
    let parameter_modes = modes_from_opcode(opcode, 3);
    let value1: i32 = value_from_parameter(parameter_modes[0], intcode, *ptr + 1);
    let value2: i32 = value_from_parameter(parameter_modes[1], intcode, *ptr + 2);
    let result_address: usize = intcode[*ptr + 3] as usize;
    let result: i32 = value1 + value2;
    //println!("Adding {} and {} to get {} to put at index {}", value1, value2, result, result_address);
    mem::replace(&mut intcode[result_address], result);
    *ptr = *ptr + 4;
}

fn multiply(intcode: &mut Vec<i32>, ptr: &mut usize, opcode: i32) {
    let parameter_modes = modes_from_opcode(opcode, 3);
    let value1: i32 = value_from_parameter(parameter_modes[0], intcode, *ptr + 1);
    let value2: i32 = value_from_parameter(parameter_modes[1], intcode, *ptr + 2);
    let result_address: usize = intcode[*ptr + 3] as usize;
    let result: i32 = value1 * value2;
    //println!("Adding {} and {} to get {} to put at index {}", value1, value2, result, result_address);
    mem::replace(&mut intcode[result_address], result);
    *ptr = *ptr + 4;
}

fn input(intcode: &mut Vec<i32>, ptr: &mut usize, _opcode: i32, input_fn: fn() -> i32) {
    let result_address: usize = intcode[*ptr + 1] as usize;
    let input = input_fn();
    //println!("Inputting {} to address {}", input, result_address);
    mem::replace(&mut intcode[result_address], input);
    *ptr = *ptr + 2;
}

fn output(intcode: &mut Vec<i32>, ptr: &mut usize, opcode: i32, output_fn: fn(i32) -> ()) {
    let parameter_modes = modes_from_opcode(opcode, 1);
    let output = value_from_parameter(parameter_modes[0], intcode, *ptr + 1);
    //let result_address: usize = intcode[*ptr + 1] as usize;
    //println!("Outputting {} from address {}", output, result_address);
    output_fn(output);
    *ptr = *ptr + 2;
}

fn jump_if_true(intcode: &mut Vec<i32>, ptr: &mut usize, opcode: i32) {
    let parameter_modes = modes_from_opcode(opcode, 2);
    let value1: i32 = value_from_parameter(parameter_modes[0], intcode, *ptr + 1);
    let value2: i32 = value_from_parameter(parameter_modes[1], intcode, *ptr + 2);
    //println!("Jump if true: Value1 {}, Value2: {}, intcode: {:?}", value1, value2, intcode);
    if value1 != 0 {
        *ptr = value2 as usize;
    } else {
        *ptr += 3;
    }
}

fn jump_if_false(intcode: &mut Vec<i32>, ptr: &mut usize, opcode: i32) {
    let parameter_modes = modes_from_opcode(opcode, 2);
    let value1: i32 = value_from_parameter(parameter_modes[0], intcode, *ptr + 1);
    let value2: i32 = value_from_parameter(parameter_modes[1], intcode, *ptr + 2);
    //println!("Jump if false: Value1 {}, Value2: {}, intcode: {:?}", value1, value2, intcode);
    if value1 == 0 {
        *ptr = value2 as usize;
    } else {
        *ptr += 3;
    }
}

fn less_then(intcode: &mut Vec<i32>, ptr: &mut usize, opcode: i32) {
    let parameter_modes = modes_from_opcode(opcode, 3);
    let value1: i32 = value_from_parameter(parameter_modes[0], intcode, *ptr + 1);
    let value2: i32 = value_from_parameter(parameter_modes[1], intcode, *ptr + 2);
    let value3: usize = intcode[*ptr + 3] as usize;
    //println!("Less than: Value1 {}, Value2: {}, Value3: {}, intcode: {:?}", value1, value2, value3, intcode);
    if value1 < value2 {
        mem::replace(&mut intcode[value3], 1);
    } else {
        mem::replace(&mut intcode[value3], 0);
    }
    *ptr = *ptr + 4;
}

fn equals(intcode: &mut Vec<i32>, ptr: &mut usize, opcode: i32) {
    let parameter_modes = modes_from_opcode(opcode, 3);
    let value1: i32 = value_from_parameter(parameter_modes[0], intcode, *ptr + 1);
    let value2: i32 = value_from_parameter(parameter_modes[1], intcode, *ptr + 2);
    let value3: usize = intcode[*ptr + 3] as usize;
    //println!("Equals: Value1 {}, Value2: {}, Value 3: {}, intcode: {:?}", value1, value2, value3, intcode);
    if value1 == value2 {
        mem::replace(&mut intcode[value3], 1);
    } else {
        mem::replace(&mut intcode[value3], 0);
    }
    *ptr = *ptr + 4;
}

fn modes_from_opcode(opcode: i32, parameters: usize) -> Vec<i32> {
    let params = opcode / 100;
    let mut results = vec![];
    let base: i32 = 10;
    for i in 0..parameters {
        let power = (i + 1) as u32;
        results.push((params % base.pow(power)) / base.pow(power-1));
    }

    return results;
}

fn value_from_parameter(parameter: i32, intcode: &Vec<i32>, ptr: usize) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_run_intcode_example1() {
        let intcode: Vec<i32> = vec![1,0,0,0,99];
        let end = vec![2,0,0,0,99];
        assert_eq!(run_intcode(intcode, || 0, |_| ()), end);
    }

    #[test]
    fn test_run_intcode_example2() {
        let intcode: Vec<i32> = vec![2,3,0,3,99];
        let end = vec![2,3,0,6,99];
        assert_eq!(run_intcode(intcode, || 0, |_| ()), end);
    }

    #[test]
    fn test_run_intcode_example3() {
        let intcode: Vec<i32> = vec![2,4,4,5,99,0];
        let end = vec![2,4,4,5,99,9801];
        assert_eq!(run_intcode(intcode, || 0, |_| ()), end);
    }

    #[test]
    fn test_run_intcode_example4() {
        let intcode: Vec<i32> = vec![1,1,1,4,99,5,6,0,99];
        let end = vec![30,1,1,4,2,5,6,0,99];
        assert_eq!(run_intcode(intcode, || 0, |_| ()), end);
    }

    #[test]
    fn test_operation_from_opcode() {
        let opcode = 1002;
        assert_eq!(operation_from_opcode(opcode), 2);
    }

    #[test]
    fn test_modes_from_opcode() {
        let opcode = 11002;
        let parameters = 3;
        assert_eq!(modes_from_opcode(opcode, parameters), vec![0, 1, 1]);

        let opcode = 01002;
        let parameters = 3;
        assert_eq!(modes_from_opcode(opcode, parameters), vec![0, 1, 0]);
    }

    #[test]
    fn test_values_from_parameters() {
        let intcode: Vec<i32> = vec![1002,4,3,4,33];
        let ptr = 1;
        let parameters: Vec<i32> = vec![0, 1, 0];
        let expected = vec![33, 3, 33];
        for (index, param) in parameters.iter().enumerate() {
            assert_eq!(
                value_from_parameter(*param, &intcode, ptr + index), 
                expected[index], 
                "\nFailed at index {} on value {}\n\n", index, param);
        }
    }

    #[test]
    fn test_add() {
        let mut intcode: Vec<i32> = vec![1002,4,3,4,33];
        let opcode = intcode[0];
        let mut ptr = 0;
        let expected = vec![1002,4,3,4,36];

        add(&mut intcode, &mut ptr, opcode);

        assert_eq!(intcode, expected);
        assert_eq!(ptr, 4);
    }

    #[test]
    fn test_multiply() {
        let mut intcode: Vec<i32> = vec![1002,4,3,4,33];
        let opcode = intcode[0];
        let mut ptr = 0;
        let expected = vec![1002,4,3,4,99];

        multiply(&mut intcode, &mut ptr, opcode);

        assert_eq!(intcode, expected);
        assert_eq!(ptr, 4);
    }

    #[test]
    fn test_input() {
        let mut intcode: Vec<i32> = vec![3,5,0,0,0,0];
        let opcode = intcode[0];
        let mut ptr = 0;
        let expected = vec![3,5,0,0,0,44];
        let input_fn = || 44;

        input(&mut intcode, &mut ptr, opcode, input_fn);

        assert_eq!(intcode, expected);
        assert_eq!(ptr, 2);
    }

    #[test]
    fn test_input_example1() {
        let mut intcode: Vec<i32> = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let opcode = intcode[0];
        let mut ptr = 0;
        let expected = vec![3,9,8,9,10,9,4,9,99,8,8];
        let input_fn = || 8;

        input(&mut intcode, &mut ptr, opcode, input_fn);

        assert_eq!(intcode, expected);
        assert_eq!(ptr, 2);
    }

    #[test]
    fn test_output() {
        let mut intcode: Vec<i32> = vec![4,5,0,0,0,33];
        let opcode = intcode[0];
        let mut ptr = 0;
        let expected = vec![4,5,0,0,0,33];
        let output_fn = |x| -> () {
            assert_eq!(x, 33);
            ()
        };

        output(&mut intcode, &mut ptr, opcode, output_fn);

        assert_eq!(intcode, expected);
        assert_eq!(ptr, 2);
    }

    #[test]
    fn test_jump_if_true_mode_position_is_true() {
        let mut intcode: Vec<i32> = vec![5,3,6,2,0,33,12];
        let opcode = intcode[0];
        let mut ptr = 0;
        let expected = 12;

        jump_if_true(&mut intcode, &mut ptr, opcode);

        assert_eq!(ptr, expected);
    }

    #[test]
    fn test_jump_if_true_mode_immidiate_is_true() {
        let mut intcode: Vec<i32> = vec![1105,1,6,0,0,33,12];
        let opcode = intcode[0];
        let mut ptr = 0;
        let expected = 6;

        jump_if_true(&mut intcode, &mut ptr, opcode);

        assert_eq!(ptr, expected);
    }

    #[test]
    fn test_jump_if_true_example6() {
        let mut intcode: Vec<i32> = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let opcode = intcode[2];
        let mut ptr = 2;
        let expected = 9;

        jump_if_true(&mut intcode, &mut ptr, opcode);

        assert_eq!(ptr, expected);
    }

    #[test]
    fn test_jump_if_false_mode_position_is_false() {
        let mut intcode: Vec<i32> = vec![6,3,6,0,0,33,12];
        let opcode = intcode[0];
        let mut ptr = 0;
        let expected = 12;

        jump_if_false(&mut intcode, &mut ptr, opcode);

        assert_eq!(ptr, expected);
    }

    #[test]
    fn test_jump_if_false_mode_immidiate_is_false() {
        let mut intcode: Vec<i32> = vec![1106,0,6,0,0,33];
        let opcode = intcode[0];
        let mut ptr = 0;
        let expected = 6;

        jump_if_false(&mut intcode, &mut ptr, opcode);

        assert_eq!(ptr, expected);
    }

    #[test]
    fn test_less_than_mode_position() {
        let mut intcode: Vec<i32> = vec![7,4,5,6,6,7,7,24];
        let opcode = intcode[0];
        let mut ptr = 0;
        let expected = vec![7,4,5,6,6,7,1,24];
        let expected_ptr = 4;

        less_then(&mut intcode, &mut ptr, opcode);

        assert_eq!(intcode, expected);
        assert_eq!(ptr, expected_ptr, "\nPointer is incorrect\n");
    }

    #[test]
    fn test_less_than_mode_immidiate() {
        let mut intcode: Vec<i32> = vec![11107,3,6,5,5,33];
        let opcode = intcode[0];
        let mut ptr = 0;
        let expected = vec![11107,3,6,5,5,1];
        let expected_ptr = 4;

        less_then(&mut intcode, &mut ptr, opcode);

        assert_eq!(intcode, expected);
        assert_eq!(ptr, expected_ptr);
    }

    #[test]
    fn test_equals_example1() {
        let mut intcode: Vec<i32> = vec![3,9,8,9,10,9,4,9,99,8,8];
        let opcode = intcode[2];
        let mut ptr = 2;
        let expected = vec![3,9,8,9,10,9,4,9,99,1,8];
        let expected_ptr = 6;

        equals(&mut intcode, &mut ptr, opcode);

        assert_eq!(intcode, expected);
        assert_eq!(ptr, expected_ptr);
    }

    #[test]
    fn test_part2_example1() {
        let intcode: Vec<i32> = vec![3,9,8,9,10,9,4,9,99,-1,8];

        let input_fn = || { 8 };
        let output_fn = |x| assert_eq!(x, 1);
        run_intcode(intcode.to_vec(), input_fn, output_fn);

        let input_fn = || { 7 };
        let output_fn = |x| assert_eq!(x, 0);
        run_intcode(intcode.to_vec(), input_fn, output_fn);
    }

    #[test]
    fn test_part2_example2() {
        let intcode: Vec<i32> = vec![3,9,7,9,10,9,4,9,99,-1,8];

        let input_fn = || { 7 };
        let output_fn = |x| assert_eq!(x, 1);
        run_intcode(intcode.to_vec(), input_fn, output_fn);

        let input_fn = || { 8 };
        let output_fn = |x| assert_eq!(x, 0);
        run_intcode(intcode.to_vec(), input_fn, output_fn);
    }

    #[test]
    fn test_part2_example3() {
        let intcode: Vec<i32> = vec![3,3,1108,-1,8,3,4,3,99];

        let input_fn = || { 8 };
        let output_fn = |x| assert_eq!(x, 1);
        run_intcode(intcode.to_vec(), input_fn, output_fn);

        let input_fn = || { 9 };
        let output_fn = |x| assert_eq!(x, 0);
        run_intcode(intcode.to_vec(), input_fn, output_fn);
    }

    #[test]
    fn test_part2_example4() {
        let intcode: Vec<i32> = vec![3,3,1107,-1,8,3,4,3,99];

        let input_fn = || { 3 };
        let output_fn = |x| assert_eq!(x, 1);
        run_intcode(intcode.to_vec(), input_fn, output_fn);

        let input_fn = || { 8 };
        let output_fn = |x| assert_eq!(x, 0);
        run_intcode(intcode.to_vec(), input_fn, output_fn);
    }

    #[test]
    fn test_part2_example5() {
        let intcode: Vec<i32> = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];

        let input_fn = || { 0 };
        let output_fn = |x| assert_eq!(x, 0);
        run_intcode(intcode.to_vec(), input_fn, output_fn);

        let input_fn = || { 8 };
        let output_fn = |x| assert_eq!(x, 1);
        run_intcode(intcode.to_vec(), input_fn, output_fn);
    }

    #[test]
    fn test_part2_example6() {
        let intcode: Vec<i32> = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];

        let input_fn = || { 0 };
        let output_fn = |x| assert_eq!(x, 0);
        run_intcode(intcode.to_vec(), input_fn, output_fn);

        let input_fn = || { 8 };
        let output_fn = |x| assert_eq!(x, 1);
        run_intcode(intcode.to_vec(), input_fn, output_fn);
    }

    #[test]
    fn test_part2_example7() {
        let intcode: Vec<i32> = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        let input_fn = || { 0 };
        let output_fn = |x| assert_eq!(x, 999);
        run_intcode(intcode.to_vec(), input_fn, output_fn);

        let input_fn = || { 8 };
        let output_fn = |x| assert_eq!(x, 1000);
        run_intcode(intcode.to_vec(), input_fn, output_fn);

        let input_fn = || { 26 };
        let output_fn = |x| assert_eq!(x, 1001);
        run_intcode(intcode.to_vec(), input_fn, output_fn);
    }
}