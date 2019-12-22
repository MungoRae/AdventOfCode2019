use std::env;
use std::fs;

mod file_loader;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_6;
mod day_7;
mod day_8;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: i32 = args[1].parse::<i32>().expect("Should provide day as first argument");
    let part: i32 = args[2].parse::<i32>().expect("Should provide part as second argument");
    let test: bool = false;

    match day {
        1 => day_one(part, test),
        2 => day_two::run(part),
        3 => day_three::run(part),
        4 => day_four::run(part),
        5 => day_five::run(part),
        6 => day_6::run(part),
        7 => day_7::run(part),
        8 => day_8::run(part),
        _ => panic!("day {} not understood", day)
    }
    
    println!("End");
}

fn day_one(part: i32, test: bool) {
    let path: String = "resources/".to_owned();

    let test_str = if test {
        "_test"
    } else {
        ""
    };
    let filename = path + "1" + test_str + ".input";
    println!("Reading from file: {:?}", filename);

    let contents: String = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let result: i32 = if part == 1 {
        contents.lines()
            .map(|l| l.parse::<i32>().unwrap())
            .map(|i| calculate_fuel(i))
            .sum()
    } else {
        let calculated: Vec<i32> = contents.lines()
            .map(|l| l.parse::<i32>().unwrap())
            .map(|i| fuel_from_mass(i))
            .collect();
        println!("Calculated values: {:?}", calculated);
        calculated.iter().sum()
    }; 

    println!("Result for part {}: {:?}", part, result);
}

fn fuel_from_mass(module_mass: i32) -> i32 {
    let mut mass: i32 = module_mass;
    let mut fuel: i32 = 0;
    loop {
        let result = calculate_fuel(mass);
        if result <= 0 {
            break;
        }
        fuel = fuel + result;
        mass = result;
        println!("result: {}, fuel: {}, mass: {}", result, fuel, mass);
    }

    return fuel;
}

fn calculate_fuel(mass: i32) -> i32 {
    let mass_f32: f32 = mass as f32;
    let result: f32 = (mass_f32 / 3.0).floor() - 2.0;
    return result as i32;
}
