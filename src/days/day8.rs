extern crate advent;
use self::advent::*;

use std::collections::HashMap;

extern crate ansi_term;
use self::ansi_term::Colour::*;


type CountsType = HashMap<char, usize>;
const LAYER_WIDTH : usize = 25;
const LAYER_HEIGHT : usize = 6;

pub fn run() {
    let filename = "inputs/day8.txt";
    let inputs = read_inputs(filename);
    let inputs = inputs.trim();

    let part_one = run_part_one(inputs, LAYER_WIDTH, LAYER_HEIGHT);
    println!("Part one: {}", part_one);

    println!("Part two:");
    let layers = split_into_layers(inputs, LAYER_WIDTH, LAYER_HEIGHT);
    let image = stack_layers(&layers);
    print_image(image.as_str(), LAYER_WIDTH, LAYER_HEIGHT);
}

fn run_part_one(input: &str, layer_width: usize, layer_height: usize) -> usize {

    let mut zero_count_indexed_counts : Vec<(usize, CountsType)> = split_into_layers(input, layer_width, layer_height)
    .iter()
    .map(|layer| digits_count(layer))
    .map(|counts| (*counts.get(&'0').unwrap(), counts))
    .collect();

    zero_count_indexed_counts.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let min_zero_layer_counts = &zero_count_indexed_counts.get(0).expect("Couldnt find elements in counts").1;
    min_zero_layer_counts[&'1'] * min_zero_layer_counts[&'2']
}

fn stack_layers(layers: &Vec<&str>) -> String {
    let mut reversed_layers = layers.clone();
    reversed_layers.reverse();

    let mut reversed_layers_iter = reversed_layers.iter();
    let mut image = reversed_layers_iter.next().expect("Layers is empty").clone().to_string();

    for layer in reversed_layers_iter {
        let image_chars = image.clone();
        let mut layer_chars = layer.chars();
        let mut image_chars = image_chars.chars();

        image = String::new();
        loop {
            let layer_char = layer_chars.next();
            let image_char = image_chars.next();

            if layer_char.is_none() {
                break;
            }

            image.push(if layer_char.unwrap() != '2' { layer_char.unwrap() } else {image_char.unwrap()});
        }
    }
    image
}

fn print_image(image: &str, layer_width: usize, layer_height: usize) {
    for i in 0..layer_height {
        for c in image.get(layer_width*i..layer_width*(i+1)).unwrap().chars() {
            if c == '0' {
                print!("{}", Black.on(Black).paint(c.to_string()));  
            } else {
                print!("{}", White.on(White).paint(c.to_string()));  
            }
        }
        println!();
    }
}

fn split_into_layers(input: &str, layer_width: usize, layer_height: usize) -> Vec<&str> {
    let mut layers = Vec::new();

    let layer_size = layer_width * layer_height;
    let number_of_layers = input.len() / layer_size;
    for i in 0..number_of_layers {
        let start = i * layer_size;
        layers.push(&input[start..start+layer_size]);
    }
    layers
}

fn digits_count(layer: &str) -> CountsType {
    let starting_count : CountsType = ('0'..='9').map(|d| (d, 0 as usize)).collect();

    let count = layer.chars().fold(starting_count, |mut counts, d| {
        let count = counts.get_mut(&d).unwrap();
        *count += 1;
        counts
    });

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_layers() {
        let input = "ABCDEFGHIJKLMNOPQR";
        let expected_layers = vec!["ABCDEF", "GHIJKL", "MNOPQR"];

        assert_eq!(expected_layers, split_into_layers(&input, 2, 3))
    }

    #[test]
    fn test_digits_count() {
        let input = "00111222233333";
        let expected_counts : CountsType = [('0', 2), ('1', 3), ('2', 4), ('3', 5), ('4', 0), ('5', 0), ('6', 0), ('7', 0), ('8', 0), ('9', 0)].iter().cloned().collect();

        assert_eq!(expected_counts, digits_count(&input));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(1, run_part_one("123456789012", 3, 2));
    }

    #[test]
    fn test_stack_layers() {
        let layers = vec!["0222", "1122", "2212", "0000"];
        let expected_image = "0110";
        assert_eq!(expected_image, stack_layers(&layers));
    }

}