extern crate advent;
use self::advent::*;

pub fn run() {
    let filename = "inputs/day5.txt";
    let inputs = split_into_vec_int(&read_inputs(filename).trim(), ",");

    let part_one = intcode::run_program(&inputs, &1);
    println!("Part one: {}", part_one.output.last().unwrap());

    let part_two = intcode::run_program(&inputs, &5);
    println!("Part two: {}", part_two.output.last().unwrap());
}