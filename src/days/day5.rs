extern crate advent;
use self::advent::*;

pub fn run() {
    let filename = "inputs/day5.txt";
    let inputs = split_into_vec_int(&read_inputs(filename).trim(), ",");

    let part_one = run_program(&inputs, &1);
    println!("Part one: {}", part_one.output.last().unwrap());

    let part_two = run_program(&inputs, &5);
    println!("Part two: {}", part_two.output.last().unwrap());
}

fn run_program(memory: &Vec<i64>, input: &i64) -> ProgramResult {
    let mut instruction_pointer = 0;
    let mut program_result = ProgramResult{memory: memory.clone(), output: vec![]};

    while get_operation(&program_result.memory, &instruction_pointer) != Operation::Halt {
        // println!("Memory: {:?} | PC: {}", program_result.memory, instruction_pointer);
        let instruction_result = run_instruction(&program_result.memory, &instruction_pointer, input);
        program_result.memory = instruction_result.memory;
        program_result.output.extend(instruction_result.output);
        instruction_pointer = instruction_result.instruction_pointer.unwrap_or(instruction_pointer + get_operation_length(&get_operation(&program_result.memory, &instruction_pointer)));
    }

    program_result
}

fn get_operation(memory: &Vec<i64>, instruction_pointer: &usize) -> Operation {
    let operation = memory.get(*instruction_pointer).expect("Couldn't get operation");
    match *operation % 100 {
        1 => Operation::Add,
        2 => Operation::Multiply,
        3 => Operation::Input,
        4 => Operation::Output,
        5 => Operation::JumpIfTrue,
        6 => Operation::JumpIfFalse,
        7 => Operation::LessThan,
        8 => Operation::Equals,
        99 => Operation::Halt,
        _ => panic!("Unknwon operation")
    }
}

fn get_operation_length(operation: &Operation) -> usize {
    match operation {
        Operation::Add => 4,
        Operation::Multiply => 4,
        Operation::Input => 2,
        Operation::Output => 2,
        Operation::JumpIfTrue => 3, 
        Operation::JumpIfFalse => 3, 
        Operation::LessThan => 4, 
        Operation::Equals => 4, 
        Operation::Halt => 1,
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

fn run_instruction(memory: &Vec<i64>, instruction_pointer: &usize, input: &i64) -> InstructionResult {
    let operation = get_operation(memory, instruction_pointer);
    let parameters = get_parameters(memory, instruction_pointer);

    let mut new_memory = memory.clone();

    let result = run_operation(&operation, &parameters, input);
    // println!("Memory: {:?} | Operation: {:?} | Parameters: {:?} | Input: {} | Result: {:?}", memory, operation, parameters, input, result);

    if let Some(new_result) = result.result {
        let index_result = get_result_address(memory, instruction_pointer);
        let result_memory = new_memory.get_mut(index_result.unwrap()).expect("Couldn't get result element");
        *result_memory = new_result;
    }

    InstructionResult {
        memory: new_memory,
        output: result.output,
        instruction_pointer: result.instruction_pointer
    }
}

fn run_operation(operation: &Operation, parameters: &Vec<i64>, input: &i64) -> OperationResult {
    return match operation {
        Operation::Add => OperationResult::with_result(parameters[0] + parameters[1]),
        Operation::Multiply=> OperationResult::with_result(parameters[0] * parameters[1]),
        Operation::Input => OperationResult::with_result(*input),
        Operation::Output => OperationResult::with_output(parameters[0]),
        Operation::JumpIfTrue => if parameters[0] != 0 { OperationResult::with_instruction_pointer(parameters[1] as usize) } else { OperationResult::EMPTY },
        Operation::JumpIfFalse => if parameters[0] == 0 { OperationResult::with_instruction_pointer(parameters[1] as usize) } else { OperationResult::EMPTY },
        Operation::LessThan => OperationResult::with_result(if parameters[0] < parameters[1] { 1 } else {0}),
        Operation::Equals => OperationResult::with_result(if parameters[0] == parameters[1] { 1 } else {0}),
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
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt
}

#[derive(Debug, PartialEq)]
struct OperationResult {
    result: Option<i64>,
    output: Option<i64>,
    instruction_pointer: Option<usize>
}

impl OperationResult {
    const EMPTY : Self = Self{result: None, output: None, instruction_pointer: None};

    pub fn with_result(result: i64) -> Self {
        Self {
            result: Some(result),
            output: None,
            instruction_pointer: None
        }
    }

    pub fn with_output(output: i64) -> Self {
        Self {
            result: None,
            output: Some(output),
            instruction_pointer: None
        }
    }

    pub fn with_instruction_pointer(ip: usize) -> Self {
        Self {
            result: None,
            output: None,
            instruction_pointer: Some(ip)
        }
    }
}

#[derive(Debug, PartialEq)]
struct InstructionResult {
    memory: Vec<i64>,
    output: Option<i64>,
    instruction_pointer: Option<usize>,
}

#[derive(Debug, PartialEq)]
struct ProgramResult {
    memory: Vec<i64>,
    output: Vec<i64>
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_program_simple() {
        let initial_memory = vec![1, 9, 10, 11, 2, 11, 11, 11, 99, 1, 2, 999];
        let expected_memory = vec![1, 9, 10, 11, 2, 11, 11, 11, 99, 1, 2, 9];

        assert_eq!(expected_memory, run_program(&initial_memory, &1).memory);
    }

    #[test]
    fn run_program_first_test_case() {
        let initial_memory = vec![1002,4,3,4,33];
        let expected_memory = vec![1002,4,3,4,99];

        assert_eq!(expected_memory, run_program(&initial_memory, &1).memory);
    }

    #[test]
    fn run_program_first_second_case() {
        let initial_memory = vec![1101,100,-1,4,0];
        let expected_memory = vec![1101,100,-1,4,99];

        assert_eq!(expected_memory, run_program(&initial_memory, &1).memory);
    }

    #[test]
    fn run_program_part_two_first_test_case() {
        // Input == 8 -> Output = 1. Else -> Output = 0
        let initial_memory = vec![3,9,8,9,10,9,4,9,99,-1,8];

        assert_eq!(vec![1], run_program(&initial_memory, &8).output);
        assert_eq!(vec![0], run_program(&initial_memory, &7).output);
    }

    #[test]
    fn run_program_part_two_second_test_case() {
        // Input < 8 -> Output = 1. Else -> Output = 0
        let initial_memory = vec![3,9,7,9,10,9,4,9,99,-1,8];

        assert_eq!(vec![1], run_program(&initial_memory, &7).output);
        assert_eq!(vec![0], run_program(&initial_memory, &8).output);
    }

    #[test]
    fn run_program_part_two_third_test_case() {
        // Input == 8 -> Output = 1. Else -> Output = 0
        let initial_memory = vec![3,3,1108,-1,8,3,4,3,99];

        assert_eq!(vec![1], run_program(&initial_memory, &8).output);
        assert_eq!(vec![0], run_program(&initial_memory, &7).output);
    }
    
    #[test]
    fn run_program_part_two_forth_test_case() {
        // Input < 8 -> Output = 1. Else -> Output = 0
        let initial_memory = vec![3,3,1107,-1,8,3,4,3,99];

        assert_eq!(vec![1], run_program(&initial_memory, &7).output);
        assert_eq!(vec![0], run_program(&initial_memory, &8).output);
    }

    #[test]
    fn run_program_part_two_fifth_test_case() {
        // Input == 0 -> Output = 0. Else -> Output = 1
        let initial_memory = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];

        assert_eq!(vec![0], run_program(&initial_memory, &0).output);
        assert_eq!(vec![1], run_program(&initial_memory, &5).output);
    }

    #[test]
    fn run_program_part_two_sixth_test_case() {
        // Input == 0 -> Output = 0. Else -> Output = 1
        let initial_memory = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];

        assert_eq!(vec![0], run_program(&initial_memory, &0).output);
        assert_eq!(vec![1], run_program(&initial_memory, &5).output);
    }

    #[test]
    fn run_program_part_two_seventh_test_case() {
        // Input < 8-> Output = 999. Input == 8 -> Output = 1000. Input > 8 -> Output = 1001
        let initial_memory = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        assert_eq!(vec![999], run_program(&initial_memory, &7).output);
        assert_eq!(vec![1000], run_program(&initial_memory, &8).output);
        assert_eq!(vec![1001], run_program(&initial_memory, &9).output);
    }

    #[test]
    fn test_get_operation() {
        let memory = vec![1, 1002, 2, 3, 4, 5, 6, 7, 8, 99];
        assert_eq!(Operation::Add, get_operation(&memory, &0));
        assert_eq!(Operation::Multiply, get_operation(&memory, &1));
        assert_eq!(Operation::Multiply, get_operation(&memory, &2));
        assert_eq!(Operation::Input, get_operation(&memory, &3));
        assert_eq!(Operation::Output, get_operation(&memory, &4));
        assert_eq!(Operation::JumpIfTrue, get_operation(&memory, &5));
        assert_eq!(Operation::JumpIfFalse, get_operation(&memory, &6));
        assert_eq!(Operation::LessThan, get_operation(&memory, &7));
        assert_eq!(Operation::Equals, get_operation(&memory, &8));
        assert_eq!(Operation::Halt, get_operation(&memory, &9));
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

        assert_eq!(expected_memory, run_instruction(&initial_memory, &0, &1).memory);
    }

    #[test]
    fn test_run_instruction_single_multiply_instruction() {
        let initial_memory = vec![2, 5, 6, 7, 99, 1, 2, 999];
        let expected_memory = vec![2, 5, 6, 7, 99, 1, 2, 2];

        assert_eq!(expected_memory, run_instruction(&initial_memory, &0, &1).memory);
    }

    #[test]
    fn test_run_instruction_output() {
        let initial_memory = vec![4, 3, 99, 5];

        assert_eq!(Some(5), run_instruction(&initial_memory, &0, &1).output);
    }


    #[test]
    fn test_run_operation() {
        assert_eq!(OperationResult::with_result(8), run_operation(&Operation::Add, &vec![6, 2], &1));
        assert_eq!(OperationResult::with_result(12), run_operation(&Operation::Multiply, &vec![6, 2], &1));
        assert_eq!(OperationResult::with_result(1), run_operation(&Operation::Input, &vec![54], &1));
        assert_eq!(OperationResult::with_output(5), run_operation(&Operation::Output, &vec![5], &1));
        assert_eq!(OperationResult::with_instruction_pointer(90), run_operation(&Operation::JumpIfTrue, &vec![1, 90], &1));
        assert_eq!(OperationResult::EMPTY, run_operation(&Operation::JumpIfTrue, &vec![0, 90], &1));
        assert_eq!(OperationResult::EMPTY, run_operation(&Operation::JumpIfFalse, &vec![1, 90], &1));
        assert_eq!(OperationResult::with_instruction_pointer(90), run_operation(&Operation::JumpIfFalse, &vec![0, 90], &1));
        assert_eq!(OperationResult::with_result(1), run_operation(&Operation::LessThan, &vec![5, 6], &1));
        assert_eq!(OperationResult::with_result(0), run_operation(&Operation::LessThan, &vec![6, 5], &1));
        assert_eq!(OperationResult::with_result(1), run_operation(&Operation::Equals, &vec![5, 5], &1));
        assert_eq!(OperationResult::with_result(0), run_operation(&Operation::Equals, &vec![5, 6], &1));
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
