use std::fmt::Write;

static INPUT: &str = include_str!("../input.txt");

fn decode_image_layers(size: (usize, usize), data: &[u8]) -> Vec<&[u8]> {
    data.chunks_exact(size.0 * size.1).collect()
}

fn part_one(layers: &[&[u8]]) {
    let min_layer = layers
        .iter()
        .min_by_key(|layer| layer.iter().filter(|&&x| x == 0).count())
        .unwrap();

    let ones = min_layer.iter().filter(|&&x| x == 1).count();
    let twos = min_layer.iter().filter(|&&x| x == 2).count();

    println!("Part one: {}", ones * twos);
}

fn show_image(size: (usize, usize), data: &[u8]) -> String {
    let mut output = String::new();

    for row in data.chunks_exact(size.0) {
        for pixel in row {
            let char = match pixel {
                0 => "  ",
                1 => "##",
                _ => "++",
            };

            write!(output, "{}", char).unwrap();
        }

        write!(output, "\n").unwrap();
    }

    output
}

fn part_two(layers: &[&[u8]]) {
    let mut output = vec![2; 25 * 6];

    for layer in layers {
        for (i, pixel) in layer.iter().enumerate() {
            if output[i] == 2 {
                output[i] = *pixel;
            }
        }
    }

    println!("Part two:\n{}", show_image((25, 6), &output));
}

fn main() {
    let input: Vec<u8> = INPUT
        .chars()
        .map(|char| char.to_digit(10).unwrap() as u8)
        .collect();

    let layers = decode_image_layers((25, 6), &input);

    part_one(&layers);
    part_two(&layers);
}
