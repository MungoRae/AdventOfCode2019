use std::collections::HashSet;
use std::collections::HashMap;

pub fn run(part: i32) {
    let result = password_number(183564, 657474, part as u32);

    println!("Result is {}", result)
}

fn password_number(start: u32, end: u32, part: u32) -> usize {
    let mut matches: HashSet<u32> = HashSet::new();
    for i in start..end {
        if match_criteria(i, part) {
            matches.insert(i);
        }
    }

    return matches.len();
}

fn match_criteria(number: u32, part: u32) -> bool {
    let mut double = false;
    let mut digit_frequencies: HashMap<u32, usize> = HashMap::new();
    let mut increasing = true;
    let as_string: String = number.to_string();
    //println!("Number: {}", as_string);
    let mut last: u32 = 0;
    for number_str in as_string.as_str().chars() {
        let number = number_str.to_digit(10).unwrap();
        //println!("char: {}", number);
        if last == number {
            double = true;
        }

        if last > number {
            increasing = false;
        }

        digit_frequencies.insert(number, digit_frequencies.get(&number).unwrap_or(&0) + 1);

        last = number;
    }

    let mut not_part_of_larger_group = false;
    for (_, frequency) in digit_frequencies {
        if frequency == 2 {
            not_part_of_larger_group = true;
            break;
        }
    }

    if part == 1 {
        return double && increasing;
    } else {
        return double && increasing && not_part_of_larger_group;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_match_criteria() {
        assert_eq!(match_criteria(223450, 1), false);
        assert_eq!(match_criteria(111111, 1), true);
        assert_eq!(match_criteria(123789, 1), false);
    }

    #[test]
    fn test_match_criteria_part_2() {
        assert_eq!(match_criteria(112233, 2), true);
        assert_eq!(match_criteria(123444, 2), false);
        assert_eq!(match_criteria(111122, 2), true);
    }
}