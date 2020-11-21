extern crate advent;
use self::advent::*;

use std::collections::HashSet;

pub fn run() {
    let filename = "inputs/day4.txt";

    let inputs = read_inputs(filename);
    let inputs : Vec<&str> = inputs.trim().split("-").collect();

    let min = inputs.get(0).expect("Couldn't get minimum value").parse::<i32>().expect("Expected integer");
    let max = inputs.get(1).expect("Couldn't get maximum value").parse::<i32>().expect("Expected integer");

    let all_passwords = calculate_passwords(6);
    let filtered_passwords = all_passwords.filter_within_range(&min, &max);
    let part_one = filtered_passwords.len();
    println!("Part one: {}", part_one);

    let filtered_passwords = filtered_passwords.filter_just_with_alone_doubles();
    let part_two = filtered_passwords.len();
    println!("Part two: {}", part_two);
}

fn calculate_passwords(max_length: usize) -> Vec<i32> {
    let mut last_passwords = vec![11, 22, 33, 44, 55, 66, 77, 88, 99];

    for _ in 2..max_length {

        let mut passwords = HashSet::new();
        for new_digit in 1..10 {

            let possible_passwords : &Vec<i32> = &last_passwords
            .iter()
            .filter(|pwd| new_digit >= last_digit(pwd))
            .map(|pwd| format!("{}{}", pwd, new_digit).parse::<i32>().unwrap())
            .collect();

            passwords.extend(possible_passwords);

            let possible_passwords : &Vec<i32> = &last_passwords
            .iter()
            .filter(|pwd| first_digit(pwd) >= new_digit)
            .map(|pwd| format!("{}{}", new_digit, pwd).parse::<i32>().unwrap())
            .collect();

            passwords.extend(possible_passwords);
    }

    last_passwords = passwords.into_iter().collect();
    }

    last_passwords.sort();
    return last_passwords;
}

fn first_digit(num: &i32) -> i32 {
    return num / (10 as i32).pow((*num as f32).log10() as u32);
}

fn last_digit(num: &i32) -> i32 {
    return num % 10;
}

fn has_alone_double(num: &i32) -> bool {
    let num_vec : Vec<char> = num.to_string().chars().collect();

    let mut has_alone_double = (num_vec[0] == num_vec[1] && num_vec[1] != num_vec[2]) || (num_vec[4] == num_vec[5] && num_vec[3] != num_vec[4]);
    for i in 1..4 {
        has_alone_double = has_alone_double || (num_vec[i-1] != num_vec[i] && num_vec[i] == num_vec[i+1] && num_vec[i+1] != num_vec[i+2]);
    }
    return has_alone_double;
}

pub trait RangeFilter {
    fn filter_within_range(&self, min: &i32, max: &i32) -> Vec<i32>;

    fn filter_just_with_alone_doubles(&self) -> Vec<i32>;
}

impl RangeFilter for Vec<i32> {
    fn filter_within_range(&self, min: &i32, max: &i32) -> Vec<i32> {
        return self.clone()
        .into_iter()
        .filter(|pwd| pwd >= &min && pwd <= &max)
        .collect();
    }

    fn filter_just_with_alone_doubles(&self) -> Vec<i32> {
        return self.clone()
        .into_iter()
        .filter(|pwd| has_alone_double(pwd))
        .collect();
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    
    #[test]
    fn test_first_digit() {
        assert_eq!(9, first_digit(&98317237));
        assert_eq!(8, first_digit(&82));
        assert_eq!(1, first_digit(&1231449999));
    }

    #[test]
    fn test_has_alone_double() {
        assert_eq!(true, has_alone_double(&112233));
        assert_eq!(false, has_alone_double(&123444));
        assert_eq!(true, has_alone_double(&111122));
    }

}