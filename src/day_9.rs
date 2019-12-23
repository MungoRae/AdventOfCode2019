use super::file_loader;

mod day_9_computer;

use day_9_computer::IntCodeComputer;

pub fn run(part: i32) {
    let input = file_loader::load_file("9.input");
    let intcode: Vec<i64> = input.split(",")
        .map(|number| number.parse::<i64>().unwrap())
        .collect();

    let result = result_for_part(&intcode, part as u32);
    println!("Result is {}", result);
}

fn result_for_part(intcode: &Vec<i64>, part: u32) -> i64 {
    let inputs = if part == 1 {
        vec![1]
    } else {
        vec![2]
    };
    let mut computer = IntCodeComputer::new(intcode, inputs);
    computer.run();
    println!("\nComputer outputs:");
    let mut last_index: i64 = -1;
    for op in &computer.outputs {
        last_index += 1;
        println!("{}", op);
    }
    return computer.outputs[last_index as usize];
}