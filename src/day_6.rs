use std::collections::HashMap;

use super::file_loader;

pub fn run(part: i32) {
    let input = file_loader::load_file("6.input");

    let result = with_input(&input, part as u32);
    
    println!("Result is {}", result);
}

fn with_input(input: &str, part: u32) -> i32 {
    let orbit_map = orbit_map(&input);

    if part == 1 {
        return calculate_total_orbits(&orbit_map) as i32;
    } else {
        return calculate_path_to_santa(&orbit_map) as i32;
    }
}

fn orbit_map(input: &str) -> HashMap<&str, &str> {
    let orbit_list: Vec<&str> = input.lines().collect();
    let mut orbit_map = HashMap::new();
    for orbit in orbit_list {
        let split: Vec<&str> = orbit.split(")").collect();
        orbit_map.insert(split[1].trim(), split[0].trim());
    }

    return orbit_map;
}

fn calculate_total_orbits(orbit_map: &HashMap<&str, &str>) -> usize {
    let mut count = 0;
    let keys: Vec<&&str> = orbit_map.keys().collect();
    for key in keys {
        let orbits = count_orbits(key, &orbit_map);
        count += orbits;
    }

    return count;
}

fn calculate_path_to_santa(orbit_map: &HashMap<&str, &str>) -> usize {
    let you_orbits = orbits("YOU", &orbit_map);
    let san_orbits = orbits("SAN", &orbit_map);

    
    for (index_you, value_you) in you_orbits.iter().enumerate() {
        for (index_san, value_san) in san_orbits.iter().enumerate() {
            if value_you == value_san {
                return index_you + index_san;
            }
        }
    }

    return 0;
}

fn count_orbits(start: &str, orbit_map: &HashMap<&str, &str>) -> usize {
    let mut next = start;
    let mut count = 0;
    while let Some(value) = orbit_map.get(next) {
        count += 1;
        next = value;
    }

    return count;
}

fn orbits<'orbitlife>(start: &str, orbit_map: &HashMap<&'orbitlife str, &'orbitlife str>) -> Vec<&'orbitlife str> {
    let mut next = start;
    let mut orbits: Vec<&str> = Vec::new();
    while let Some(value) = orbit_map.get(next) {
        orbits.push(value);
        next = value;
    }

    println!("Orbits for {} is {:?}", start, orbits);

    return orbits;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example() {
        let input = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L";

        let expected = 42;

        assert_eq!(with_input(input, 1), expected);
    }

    #[test]
    fn test_example_part_2() {
        let input = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN";

        let expected = 4;

        assert_eq!(with_input(input, 2), expected);
    }
}