extern crate advent;
use self::advent::*;

pub fn run() {
    let filename = "inputs/day1.txt";
    
    let inputs = split_lines_into_vec_int(&read_inputs(filename));
    let part_one = run_function_and_sum_all(calculate_fuel, &inputs);
    println!("Part One: {}", part_one);

    let part_two = run_function_and_sum_all(calculate_fuel_including_own_fuel_mass, &inputs);
    println!("Part Two: {}", part_two);
}

fn calculate_fuel(module: &i64) -> i64 {
    return module / 3 - 2;
}

fn calculate_fuel_including_own_fuel_mass(module: &i64) -> i64 {
    let fuel = calculate_fuel(module);
    let adjusted_fuel = if fuel > 0 {
        calculate_fuel_including_own_fuel_mass(&fuel)
    } else {
        0
    };
    let adjusted_fuel = if adjusted_fuel > 0 { adjusted_fuel } else { 0 };
    return fuel + adjusted_fuel;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_fuel_simple() {
        assert_eq!(2, calculate_fuel(&12))
    }
    
    #[test]
    fn test_calculate_fuel_with_round_down() {
        assert_eq!(2, calculate_fuel(&14))
    }
    
    #[test]
    fn test_calculate_fuel_complex() {
        assert_eq!(33583, calculate_fuel(&100756))
    }
    
    #[test]
    fn test_calculate_fuel_including_own_fuel_mass_simple() {
        assert_eq!(2, calculate_fuel_including_own_fuel_mass(&14));
    }
    
    #[test]
    fn test_calculate_fuel_including_own_fuel_mass_complex() {
        assert_eq!(50346, calculate_fuel_including_own_fuel_mass(&100756));
    }
}