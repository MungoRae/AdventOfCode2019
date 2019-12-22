use super::file_loader;

const WIDTH_PIXELS: usize = 25;
const HEIGHT_PIXELS: usize = 6;

pub fn run(part: i32) {
    let input = file_loader::load_file("8.input");
    let result = result_for_part(&input, part as u32);
    println!("Result is {}", result);
}

fn result_for_part(input: &str, part: u32) -> i32 {
    //println!("Input: {}", input);
    let data : Vec<u32> = input.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let image = make_layers(&data, WIDTH_PIXELS, HEIGHT_PIXELS);
    

    if part == 1 {
        return part_one(&image);
    } else {
        return part_two(&image);
    }
}

fn part_two(layered_image: &Vec<Vec<Vec<u32>>>) -> i32 {
    let image = resolve_layers(layered_image);
    display_image(&image);
    return 0;
}

fn part_one(image: &Vec<Vec<Vec<u32>>>) -> i32 {
    let mut fewest_zero_digits = -1;
    let mut fewest_zero_digits_layer = 0;

    for (index, layer) in image.iter().enumerate() {
        let mut zero_digits = 0;
        for row in layer {
            for pixel in row {
                if *pixel == 0 {
                    zero_digits += 1;
                }
            }
        }

        if fewest_zero_digits == -1 || zero_digits < fewest_zero_digits {
            fewest_zero_digits = zero_digits;
            fewest_zero_digits_layer = index;
        }
    }

    let mut ones = 0;
    let mut twos = 0;
    for row in &image[fewest_zero_digits_layer] {
        for pixel in row {
            if *pixel == 1 {
                ones += 1;
            } else if *pixel == 2 {
                twos += 1;
            }
        }
    }

    return ones * twos;
}

fn resolve_layers(layered_image: &Vec<Vec<Vec<u32>>>) -> Vec<Vec<u32>> {
    let mut image: Vec<Vec<u32>> = layered_image.get(0).unwrap().to_vec();

    for layer in layered_image {
        for (row_index, row) in layer.iter().enumerate() {
            for (pixel_index, pixel) in row.iter().enumerate() {
                let top = image[row_index][pixel_index];
                let bottom = pixel;
                if top == 2 {
                    image[row_index][pixel_index] = *bottom;
                }
            }
        }
    }

    return image;
}

fn display_image(image: &Vec<Vec<u32>>) {
    println!("\nImage:");
    for row in image {
        let row_out = row.iter().fold("".to_owned(), |pixel_string, pixel| fold_pixels_to_string(pixel_string, pixel));
        println!("{}", row_out);
    }

    println!("\n");
}

fn fold_pixels_to_string(pixel_string: String, pixel: &u32) -> String {
    let character = match *pixel {
        0 => ' ',
        1 => 'W',
        2 => 'T',
        _ => panic!("\nPixel {} not understood\n")
    };
    return format!("{}{}", pixel_string, character);
}

fn make_layers(data: &Vec<u32>, width: usize, height: usize) -> Vec<Vec<Vec<u32>>> {
    let mut image: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut ptr = 0;

    loop {
        let mut layer = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                let pixel: u32 = data[ptr];
                ptr += 1;
                row.push(pixel);
            }
            layer.push(row);
        }

        image.push(layer);
        if data.get(ptr) == None {
            println!("We have {} layers", image.len());
            break;
        }
    }

    return image;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day8_part1() {
        let input = vec![1,2,3,4,5,6,7,8,9,0,1,2];
        let expected = vec![
                [
                    [1,2,3],
                    [4,5,6]
                ],
                [
                    [7,8,9],
                    [0,1,2]
                ]
            ];

        assert_eq!(make_layers(&input, 3, 2), expected);
    }

    #[test]
    fn test_day8_part2() {
        let input = vec![0,2,2,2,1,1,2,2,2,2,1,2,0,0,0,0];
        let expected = vec![
                [0,1],
                [1,0],
            ];

        let image = make_layers(&input, 2, 2);

        assert_eq!(resolve_layers(&image), expected);
    }
}