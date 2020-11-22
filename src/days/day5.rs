extern crate advent;
use self::advent::*;

pub fn run() {
    let filename = "inputs/day5.txt";
    let inputs = split_into_vec_int(&read_inputs(filename).trim(), ",");

    run_program(&inputs);
    println!();
}

fn run_program(memory: &Vec<i64>) -> Vec<i64> {
    let mut instruction_pointer = 0;
    let mut memory = memory.clone();

    while get_operation(&memory, &instruction_pointer) != Operation::Halt {
        memory = run_instruction(&memory, &instruction_pointer);
        instruction_pointer = instruction_pointer + get_operation_length(&get_operation(&memory, &instruction_pointer));
    }
    return memory;
}

fn get_operation(memory: &Vec<i64>, instruction_pointer: &usize) -> Operation {
    let operation = memory.get(*instruction_pointer).expect("Couldn't get operation");
    match *operation % 100 {
        1 => Operation::Add,
        2 => Operation::Multiply,
        3 => Operation::Input,
        4 => Operation::Output,
        _ => Operation::Halt
    }
}

fn get_operation_length(operation: &Operation) -> usize {
    match operation {
        Operation::Add => 4,
        Operation::Multiply => 4,
        Operation::Input => 2,
        Operation::Output => 2,
        _ => 1
    }
}

fn get_modes(memory: &Vec<i64>, instruction_pointer: &usize) -> Vec<Mode> {
    let operation_length = get_operation_length(&get_operation(memory, instruction_pointer));
    let operation = memory.get(*instruction_pointer).expect("Couldn't get operation").to_string();
    let operation = format!("{:0>width$}", operation, width=operation_length+1);
    let modes : Vec<Mode> = operation[..operation.len()-2].chars().into_iter().filter_map(|c| to_mode(&c)).rev().collect();
    return modes;
}

fn to_mode(mode_str: &char) -> Option<Mode> {
    match mode_str {
        '0' => Some(Mode::Position),
        '1' => Some(Mode::Immediate),
        _ => None 
    }
}

fn get_parameters(memory: &Vec<i64>, instruction_pointer: &usize) -> Vec<i64> {
    let modes = get_modes(memory, instruction_pointer);
    let mut parameter_idx = instruction_pointer + 1;

    modes.iter()
    .map(|mode| {
        let first_parameter = memory.get(parameter_idx).expect("Couldnt get parameter").clone();
        parameter_idx = parameter_idx + 1;
        match mode {
            Mode::Immediate => first_parameter,
            Mode::Position => memory.get(first_parameter as usize).expect("Couldnt get parameter at address").clone()
        }
    }).collect()
}

fn get_result_address(memory: &Vec<i64>, instruction_pointer: &usize) -> Option<usize> {
    let operation_length = get_operation_length(&get_operation(memory, instruction_pointer));
    if operation_length == 1 { 
        None
    } else { 
        Some(*memory.get(instruction_pointer + operation_length - 1).expect("Couldn't get address index") as usize)
    }
}

fn run_instruction(memory: &Vec<i64>, instruction_pointer: &usize) -> Vec<i64> {
    // println!("Memory: {:?} | PC: {}", memory, instruction_pointer);
    let operation = get_operation(memory, instruction_pointer);
    let parameters = get_parameters(memory, instruction_pointer);

    let mut new_memory = memory.clone();

    if let Some(result) = run_operation(&operation, &parameters) {
        let index_result = get_result_address(memory, instruction_pointer);
        let result_memory = new_memory.get_mut(index_result.unwrap()).expect("Couldn't get result element");
        *result_memory = result;
    }

    return new_memory;
}

fn run_operation(operation: &Operation, parameters: &Vec<i64>) -> Option<i64> {
    return match operation {
        Operation::Add => Some(parameters[0] + parameters[1]),
        Operation::Multiply=> Some(parameters[0] * parameters[1]),
        Operation::Input => Some(1),
        Operation::Output => { print!("{}", parameters[0]); return None},
        _ => panic!("Unknown operation")
    };
}

#[derive(Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    Halt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_program_simple() {
        let initial_memory = vec![1, 9, 10, 11, 2, 11, 11, 11, 99, 1, 2, 999];
        let expected_memory = vec![1, 9, 10, 11, 2, 11, 11, 11, 99, 1, 2, 9];

        assert_eq!(expected_memory, run_program(&initial_memory));
    }

    #[test]
    fn run_program_first_test_case() {
        let initial_memory = vec![1002,4,3,4,33];
        let expected_memory = vec![1002,4,3,4,99];

        assert_eq!(expected_memory, run_program(&initial_memory));
    }

    #[test]
    fn run_program_first_second_case() {
        let initial_memory = vec![1101,100,-1,4,0];
        let expected_memory = vec![1101,100,-1,4,99];

        assert_eq!(expected_memory, run_program(&initial_memory));
    }

    #[test]
    fn test_get_operation() {
        let memory = vec![1, 1002, 2, 3, 4, 99];
        assert_eq!(Operation::Add, get_operation(&memory, &0));
        assert_eq!(Operation::Multiply, get_operation(&memory, &1));
        assert_eq!(Operation::Multiply, get_operation(&memory, &2));
        assert_eq!(Operation::Input, get_operation(&memory, &3));
        assert_eq!(Operation::Output, get_operation(&memory, &4));
        assert_eq!(Operation::Halt, get_operation(&memory, &5));
    }

    #[test]
    fn test_get_modes() {
        assert_eq!(vec![Mode::Position, Mode::Immediate, Mode::Immediate], get_modes(&vec![11002], &0));
        assert_eq!(vec![Mode::Position, Mode::Position, Mode::Position], get_modes(&vec![2], &0));
    }

    #[test]
    fn test_get_parameters() {
        assert_eq!(vec![5, 6, 100], get_parameters(&vec![1101, 5, 6, 7, 99, 1, 2, 100], &0));
        assert_eq!(vec![1, 2, 100], get_parameters(&vec![1, 5, 6, 7, 99, 1, 2, 100], &0));
    }

    #[test]
    fn test_run_instruction_single_sum_instruction() {
        let initial_memory = vec![1, 5, 6, 7, 99, 1, 2, 999];
        let expected_memory = vec![1, 5, 6, 7, 99, 1, 2, 3];

        assert_eq!(expected_memory, run_instruction(&initial_memory, &0));
    }

    #[test]
    fn test_run_instruction_single_multiply_instruction() {
        let initial_memory = vec![2, 5, 6, 7, 99, 1, 2, 999];
        let expected_memory = vec![2, 5, 6, 7, 99, 1, 2, 2];

        assert_eq!(expected_memory, run_instruction(&initial_memory, &0));
    }

    #[test]
    fn test_run_operation() {
        assert_eq!(Some(8), run_operation(&Operation::Add, &vec![6, 2]));
        assert_eq!(Some(12), run_operation(&Operation::Multiply, &vec![6, 2]));
        assert_eq!(Some(1), run_operation(&Operation::Input, &vec![54]));
        assert_eq!(None, run_operation(&Operation::Output, &vec![5]));
    }

    #[test]
    fn test_get_operation_length() {
        assert_eq!(4, get_operation_length(&Operation::Add));
        assert_eq!(4, get_operation_length(&Operation::Multiply));
        assert_eq!(2, get_operation_length(&Operation::Input));
        assert_eq!(2, get_operation_length(&Operation::Output));
        assert_eq!(1, get_operation_length(&Operation::Halt));
    }

    #[test]
    fn test_get_result_address() {
        assert_eq!(Some(92), get_result_address(&vec![1, 90, 91, 92], &0));
        assert_eq!(Some(92), get_result_address(&vec![2, 90, 91, 92], &0));
        assert_eq!(Some(92), get_result_address(&vec![3, 92], &0));
        assert_eq!(Some(92), get_result_address(&vec![4, 92], &0));
        assert_eq!(None, get_result_address(&vec![99], &0));
    }

}
