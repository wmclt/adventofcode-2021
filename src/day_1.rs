use std::vec::Vec;

use crate::util::read_data;

pub fn part_1() -> u32 {
    let file_path = "./data/day_1.txt";
    let mut numbers = read_data(file_path, parse_i32);
    count_increases(&mut numbers)
}

pub fn part_2() -> u32 {
    let file_path = "./data/day_1.txt";
    let numbers = read_data(file_path, parse_i32);
    let mut window_numbers = sum_windows(numbers);
    count_increases(&mut window_numbers)
}

fn sum_windows(numbers: Vec<i32>) -> Vec<i32> {
    let mut window_sums = Vec::new();
    for i in 0..numbers.len() - 2 {
        window_sums.push(numbers[i] + numbers[i + 1] + numbers[i + 2]);
    }
    window_sums
}

fn count_increases(numbers: &mut Vec<i32>) -> u32 {
    let mut nr_increases = 0;
    let mut prev_nbr = numbers.pop().unwrap();

    while let Some(nbr) = numbers.pop() {
        if nbr < prev_nbr {
            nr_increases += 1;
        }
        prev_nbr = nbr;
    }
    nr_increases
}

fn parse_i32(line: String) -> i32 {
    line.parse().expect("Failed to parse int!")
}

#[cfg(test)]
mod tests {

    use crate::day_1::{count_increases, sum_windows};

    #[test]
    fn test_part_1() {
        let mut numbers = vec![1, 2, 3, 4, 3];
        assert_eq!(count_increases(&mut numbers), 3);
    }

    #[test]
    fn other_test_part_1() {
        let mut numbers = vec![32, 21, 25, 226, 321, 488, 531];
        assert_eq!(count_increases(&mut numbers), 5);
    }

    #[test]
    fn test_sum_windows() {
        let numbers = vec![1, 2, 3, 4, 6, 4, 4, 4];
        assert_eq!(sum_windows(numbers), vec![6, 9, 13, 14, 14, 12]);
    }

    #[test]
    fn test_part_2() {
        let numbers = vec![1, 2, 3, 4, 6, 4, 4, 4];
        assert_eq!(count_increases(&mut sum_windows(numbers)), 3);
    }
}
