use std::collections::HashSet;

use super::file_loader;

pub fn run(part: i32) {
    let input = file_loader::load_file("10.input");
    let result = from_text(&input, part as u32);
    println!("Result is {}", result);
}

fn from_text(input: &str, _part: u32) -> usize {
    println!("Input: ");
    println!("{}", input);
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let row_size = grid.len();
    let column_size = grid[0].len();
    
    let mut most = 0;
    for r in 0..row_size {
        for s in 0..column_size {
            if grid[r][s] != '#' {
                continue;
            }

            //println!("Looking at pos {},{}", r, s);

            let mut seen: HashSet<(i64, i64)> = HashSet::new();
            for rr in 0..row_size {
                for ss in 0..column_size {
                    if grid[rr][ss] == '#' && (r != rr || s != ss) {
                        let dr = rr as i64 - r as i64;
                        let dc = ss as i64 - s as i64;
                        let gcd: i64 = gcd(dr, dc).abs();
                        let sr = dr/gcd;
                        let sc = dc/gcd;
                        //println!("from: {},{} to {},{}: diff: {},{}, gcd (abs) {}, shortest: {},{}", r, s, rr, ss, dr, dc, gcd, sr, sc);
                        seen.insert((sr, sc));
                    }
                }
            }

            //println!("Seen: Pos: {},{}. len: {}. {:?}", r, s, seen.len(), seen);
            if seen.len() > most {
                most = seen.len();
            }
        }
    }

    return most;
}

fn gcd(x: i64, y: i64) -> i64 {
    if x == 0 {
        return y;
    } else {
        return gcd(y % x, x);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day10_example1() {
        let input = 
".#..#
.....
#####
....#
...##";

        let expected = 8;

        assert_eq!(from_text(&input, 1), expected);
    }

    #[test]
    fn test_day10_example2() {
        let input = 
"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

        let expected = 33;

        assert_eq!(from_text(&input, 1), expected);
    }

    #[test]
    fn test_day10_example3() {
        let input = 
"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

        let expected = 35;

        assert_eq!(from_text(&input, 1), expected);
    }

    #[test]
    fn test_day10_example4() {
        let input = 
".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

        let expected = 41;

        assert_eq!(from_text(&input, 1), expected);
    }

    #[test]
    fn test_day10_example5() {
        let input = 
".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        let expected = 210;

        assert_eq!(from_text(&input, 1), expected);
    }
}