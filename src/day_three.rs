use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use super::file_loader;

pub fn run(part: i32) {
    let input = file_loader::load_file("3.input");
    //println!("File content: {:?}\n", input);
    let result = with_input(&input, part as u32);

    println!("Answer = {}", result);
}

fn with_input(input: &str, part: u32) -> i32 {
    let paths: Vec<&str> = input.lines().collect();

    //println!("Paths: {:?}\n", paths);

    let direction_map: HashMap<&str, (i32, i32)> = [
        ("L", (-1,  0)),
        ("R", ( 1,  0)),
        ("U", ( 0,  1)),
        ("D", ( 0, -1))
    ].iter().cloned().collect();

    let mut path_points: Vec<HashMap<usize, (i32, i32)>> = Vec::new();
    for path in paths {
        let elements: Vec<&str> = path.split(',').collect();
        //println!("Elements: {:?}", elements);
        let mut points: HashMap<usize, (i32, i32)> = HashMap::new();
        let mut current: (i32, i32) = (0, 0);
        let mut steps = 0;
        for item in elements {
            let dir = &item[0..1];
            let dir_tuple = direction_map[dir];
            let amount_str: &str = &item[1..];
            //println!("Amount string: {:?}", amount_str);
            let amount: i32 = amount_str.parse::<i32>().unwrap();
            //println!("Direction: {}, amount: {}", dir, amount);
            for _ in 0..amount {
                steps += 1;
                current = (current.0 + dir_tuple.0, current.1 + dir_tuple.1);
                points.insert(steps, current);
                //println!("Inserting: {:?}", current);
            }
            //println!("Points: {:?}", points);
        }
        path_points.push(points);
    }

    if part == 1 {
        let points1: HashSet<&(i32, i32)> = HashSet::from_iter(path_points[0].values());
        let points2: HashSet<&(i32, i32)> = HashSet::from_iter(path_points[1].values());
        let intersections = points1.intersection(&points2);
        //println!("Intersections: {:?}", intersections);
        let manhatten_distance: i32 = intersections
            .map(|pp| (pp.0.abs(), pp.1.abs()))
            .map(|pp| (pp.0 + pp.1))
            .min()
            .unwrap();
    
        return manhatten_distance;
    } else {
        let map_one: &HashMap<usize, (i32, i32)> = &path_points[0];
        let map_two: &HashMap<usize, (i32, i32)> = &path_points[1];

        let map_one_values: HashSet<&(i32, i32)> = HashSet::from_iter(map_one.values());
        let map_two_values: HashSet<&(i32, i32)> = HashSet::from_iter(map_two.values());
        let intersections = map_one_values.intersection(&map_two_values);
        
        let mut lowest = 0;
        for i in intersections {
            let mut steps_one: usize = 0;
            let mut steps_two: usize = 0;
            for (key, value) in map_one.iter() {
                if value == *i {
                    steps_one = *key;
                }
            }
            
            for (key, value) in map_two.iter() {
                if value == *i {
                    steps_two = *key;
                }
            }

            if lowest == 0 || steps_one + steps_two < lowest {
                lowest = steps_one + steps_two;
            }
        }
        
        return lowest as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example1_part1() {
        assert_eq!(with_input("R8,U5,L5,D3\nU7,R6,D4,L4", 1), 6);
    }

    #[test]
    fn example2_part1() {
        assert_eq!(with_input("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83", 1), 159);
    }

    #[test]
    fn example3_part1() {
        assert_eq!(with_input("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7", 1), 135);
    }

    #[test]
    fn example1_part2() {
        assert_eq!(with_input("R8,U5,L5,D3\nU7,R6,D4,L4", 2), 30);
    }

    #[test]
    fn example2_part2() {
        assert_eq!(with_input("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83", 2), 610);
    }

    #[test]
    fn example3_part2() {
        assert_eq!(with_input("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7", 2), 410);
    }
}