use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let file = read_to_string("src/bin/day08-space-image-format/input.txt")?;
    let input = file.trim();

    println!("{}", compute_part1_result(input, &6, &25));
    println!("{}", compute_part2_result(input, &6, &25));

    Ok(())
}

fn compute_part1_result(input: &str, height: &i32, width: &i32) -> usize {
    let layer = find_layer_with_fewest_0_digits(input, height, width);
    let number_of_1_digits = layer.iter().filter(|&c| *c == '1').count();
    let number_of_2_digits = layer.iter().filter(|&c| *c == '2').count();
    number_of_1_digits * number_of_2_digits
}

fn compute_part2_result(input: &str, height: &i32, width: &i32) -> String {
    let image = merge_layers(input, height, width);
    let mut output: String = String::new();
    for h in 0..height.clone() {
        output.push('8');
        for w in 0..width.clone() {
            let ind: usize = (h * width + w) as usize;
            output.push(image[ind]);
        }
        output.push('\n');
    }
    output
}

fn find_layer_with_fewest_0_digits(input: &str, height: &i32, width: &i32) -> Vec<char> {
    let layers = parse_layers(input, height, width);
    let mut iter = layers.iter();
    let mut min_number_of_0_digits = usize::max_value();
    let mut min_number_of_0_digits_layer: Vec<char> = vec![];
    loop {
        match iter.next() {
            Some(layer) => {
                let number_of_0_digits = layer.iter().filter(|&c| *c == '0').count();
                if number_of_0_digits < min_number_of_0_digits {
                    min_number_of_0_digits = number_of_0_digits;
                    min_number_of_0_digits_layer = layer.clone();
                }
            }
            _ => break,
        }
    }
    min_number_of_0_digits_layer
}

fn merge_layers(input: &str, height: &i32, width: &i32) -> Vec<char> {
    let layers = parse_layers(input, height, width);
    let mut image = Vec::new();
    for h in 0..height.clone() {
        for w in 0..width.clone() {
            let mut iter = layers.iter();
            loop {
                match iter.next() {
                    Some(layer) => {
                        let ind: usize = (h * width + w) as usize;
                        match layer[ind] {
                            '0' => {
                                image.push('8');
                                break;
                            }
                            '1' => {
                                image.push(' ');
                                break;
                            }
                            _ => continue,
                        }
                    }
                    _ => {
                        println!("All layers are transparent ! Fallback to white");
                        image.push(' ');
                    }
                }
            }
        }
    }
    image
}

fn parse_layers(input: &str, height: &i32, width: &i32) -> Vec<Vec<char>> {
    let mut layers = Vec::new();
    let mut chars = input.chars();
    'outer: loop {
        let mut layer = Vec::new();
        for _ in 0..height.clone() {
            for _ in 0..width.clone() {
                match chars.next() {
                    Some(character) => layer.push(character),
                    _ => break 'outer,
                }
            }
        }
        layers.push(layer);
    }
    layers
}

#[cfg(test)]
mod tests {
    use super::merge_layers;
    use super::parse_layers;

    #[test]
    fn it_should_parse_input_layers() {
        let layers = parse_layers("123456789012", &2, &3);

        assert_eq!(
            layers,
            vec![
                vec!['1', '2', '3', '4', '5', '6'],
                vec!['7', '8', '9', '0', '1', '2'],
            ]
        );
    }

    #[test]
    fn it_should_merge_layers() {
        let image = merge_layers("0222112222120000", &2, &2);
        assert_eq!(image, vec!['8', ' ', ' ', '8']);
    }
}
