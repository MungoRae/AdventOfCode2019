use std::mem;
use std::env;

use super::file_loader;

pub fn run(part: i32) {
    let input = file_loader::load_file("2.input");
    println!("File content: {:?}", input);

    let intcode: Vec<u32> = input.split(",")
        .map(|number| number.parse::<u32>().unwrap())
        .collect();

    if part == 1 {
        let args: Vec<String> = env::args().collect();
        let noun = args[3].parse::<u32>().expect("Should provide noun");
        let verb = args[4].parse::<u32>().expect("Should provide verb");

        let result = process_for_result(intcode.to_vec(), noun, verb);
        println!();
        println!("Answer is {}", result);
    } else {
        'outer: for i in 30..60 {
            'inner: for j in 30..50 {
                let result = process_for_result(intcode.to_vec(), i, j);
                println!("Result for n {} and v {} is {}", i, j, result);
                if result == 19690720 {
                    println!();
                    println!("Answer is {}", 100 * i + j);
                    break 'outer;
                }
            }
        }
    }
    
}

fn process_for_result(mut intcode: Vec<u32>, noun: u32, verb: u32) -> u32 {
    //println!("Processing: {:?}", intcode);
    intcode[1] = noun;
    intcode[2] = verb;

    let mut ptr: usize = 0;
    loop {
        //println!("Pointer is: {}, Value is: {}", ptr, intcode[ptr]);
        //println!("Before: {:?}", intcode);
        process_next_number(&mut intcode, &mut ptr);
        if intcode[ptr] == 99 {
            break;
        }
    }

    //println!("Final: {:?}", intcode);

    return intcode[0];
}

fn process_next_number(run: &mut Vec<u32>, ptr: &mut usize) {
    let opcode = run[*ptr];
    
    match opcode {
        1 => add(run, ptr),
        2 => multiply(run, ptr),
        _ => panic!("Unknown opcode {}. Something went wrong", opcode)
    }
}

fn add(run: &mut Vec<u32>, ptr: &mut usize) {
    let one: usize = run[*ptr + 1] as usize;
    let two: usize = run[*ptr + 2] as usize;
    let answer: usize = run[*ptr + 3] as usize;
    let result = run[one] + run[two];
    //println!("Adding {} and {} to get {} to put at index {}", run[one], run[two], result, answer);
    //run.insert(answer, result);
    mem::replace(&mut run[answer], result);
    //let mut newValue: usize = *ptr + 4;
    *ptr = *ptr + 4;
}

fn multiply(run: &mut Vec<u32>, ptr: &mut usize) {
    let one: usize = run[*ptr + 1] as usize;
    let two: usize = run[*ptr + 2] as usize;
    let answer: usize = run[*ptr + 3] as usize;
    let result = run[one] * run[two];
    //println!("Adding {} and {} to get {} to put at index {}", run[one], run[two], result, answer);
    //run.insert(answer, result);
    mem::replace(&mut run[answer], result);
    //let mut newValue: usize = *ptr + 4;
    *ptr = *ptr + 4;
}