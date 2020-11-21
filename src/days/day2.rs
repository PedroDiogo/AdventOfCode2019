extern crate advent;
use self::advent::*;

pub fn run() {
    let filename = "inputs/day2.txt";
    let inputs = split_into_vec_usize(&read_inputs(filename).trim(), ",");

    let part_one = run_program_changing_params(&inputs, &12, &2);
    println!("Part One: {}", part_one);

    let part_two = find_noun_verb_calc_for_output(&inputs, &19690720);
    println!("Part Two: {}", part_two);
}

fn run_program_changing_params(inputs: &Vec<usize>, noun: &usize, verb: &usize) -> usize {
    let mut program = inputs.clone();
    *program.get_mut(1).unwrap() = *noun;
    *program.get_mut(2).unwrap() = *verb;

    let program = run_program(&program);
    return *program.get(0).unwrap();
}

fn find_noun_verb_calc_for_output(inputs: &Vec<usize>, output: &usize) -> usize {
    let mut noun = 99;
    let mut verb = 99;
    while run_program_changing_params(&inputs, &noun, &verb) != *output {
        if verb == 0 {
            noun = noun - 1;
            verb = 99;
        } else {
            verb = verb - 1;
        }
    }

    return 100 * noun + verb;
}

fn run_program(memory: &Vec<usize>) -> Vec<usize> {
    let mut instruction_pointer = 0;
    let mut memory = memory.clone();

    while !is_halt(&memory, &instruction_pointer) {
        memory = run_instruction(&memory, &instruction_pointer);
        instruction_pointer = instruction_pointer + 4;
    }
    return memory;
}

fn is_halt(memory: &Vec<usize>, instruction_pointer: &usize) -> bool {
    let operation = memory.get(*instruction_pointer).expect("Couldn't get operation");

    return *operation == 99;
}

fn run_instruction(memory: &Vec<usize>, instruction_pointer: &usize) -> Vec<usize> {
    // println!("Memory: {:?} | PC: {}", memory, instruction_pointer);
    let operation = memory.get(*instruction_pointer).expect("Couldn't get operation");
    let index_1 = memory.get(instruction_pointer + 1).expect("Couldn't get first index");
    let index_2 = memory.get(instruction_pointer + 2).expect("Couldn't get second index");
    let index_result = memory.get(instruction_pointer + 3).expect("Couldn't get second index");
    let element_1 = memory.get(*index_1).expect("Coulnd't get first element");
    let element_2 = memory.get(*index_2).expect("Coulnd't get first element");

    let mut new_memory = memory.clone();
    let result = new_memory.get_mut(*index_result).expect("Couldn't get result element");

    *result = run_operation(operation, element_1, element_2);

    return new_memory;
}

fn run_operation(operation: &usize, element_1: &usize, element_2: &usize) -> usize {
    return match operation {
        1 => element_1 + element_2,
        2 => element_1 * element_2,
        _ => panic!("Unknown operation")
    };
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
    fn test_is_halt_true() {
        let memory = vec![0, 1, 2, 3, 99];
        assert_eq!(true, is_halt(&memory, &4))
    }

    #[test]
    fn test_is_halt_false() {
        let memory = vec![0, 1, 2, 3, 99];
        assert_eq!(false, is_halt(&memory, &0))
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
    fn test_run_operation_sum() {
        assert_eq!(8, run_operation(&1, &6, &2));
    }

    #[test]
    fn test_run_operation_multiply() {
        assert_eq!(12, run_operation(&2, &6, &2));
    }

    #[test]
    #[should_panic(expected="Unknown operation")]
    fn test_run_operation_unknown_operation() {
        run_operation(&3, &6, &2);
    }
}