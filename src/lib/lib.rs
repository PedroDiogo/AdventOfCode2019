use std::fs; 

pub fn read_inputs(filename: &str) -> String {
    return fs::read_to_string(filename)
    .expect("Couldn't read file");
}

pub fn split_lines_into_vec_str(input: &str) -> Vec<&str> {
    input.split("\n")
    .collect()
}

pub fn split_lines_into_vec_int(input: &str) -> Vec<i64> {
    return split_into_vec_int(input, "\n");
}

pub fn split_into_vec_int(input: &str, delimiter: &str) -> Vec<i64> {
    return input.split(delimiter)
    .into_iter()
    .filter(|&line| line.ne(""))
    .map(|line| line.parse::<i64>().expect("Couldn't convert to i64"))
    .collect()
}

pub fn split_into_vec_usize(input: &str, delimiter: &str) -> Vec<usize> {
    return input.split(delimiter)
    .into_iter()
    .filter(|&line| line.ne(""))
    .map(|line| line.parse::<usize>().expect("Couldn't convert to i64"))
    .collect()
}

pub fn run_function_and_sum_all(f: fn(&i64) -> i64, elements: &Vec<i64>) -> i64 {
    return elements.into_iter()
    .map(|input| f(input))
    .fold(0, |acc, elem| acc + elem);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_lines_into_vec_int() {
        let input = "1\n2\n999\n";

        assert_eq!(vec![1,2,999], split_lines_into_vec_int(input));
    }

    #[test]
    fn test_split_into_vec_int() {
        let input = "1 | 2 | 999";
        let delimiter = " | ";

        assert_eq!(vec![1,2,999], split_into_vec_int(input, &delimiter));
    }

    #[test]
    fn test_split_into_vec_usize() {
        let input = "1 | 2 | 999";
        let delimiter = " | ";

        assert_eq!(vec![1,2,999], split_into_vec_usize(input, &delimiter));
    }
}