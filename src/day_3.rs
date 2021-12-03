use std::vec::Vec;

use crate::util::read_data;

pub fn part_1() -> u32 {
    let file_path = "./data/day_3.txt";
    let instructions = read_data(file_path, line_to_chars);
    count(instructions)
}

//iteratively keep most frequent until only one remains
//iteratively keep least frequent until only one remains
// multiply both numbers
pub fn part_2() -> u32 {
    let file_path = "./data/day_3.txt";
    let char_matrix = read_data(file_path, line_to_chars);
    let char_matrix_2 = char_matrix.clone();

    let oxygen = calc_oxygen(char_matrix);
    let co2 = calc_co2(char_matrix_2);
    oxygen * co2
}

fn calc_oxygen(mut char_matrix: Vec<Vec<char>>) -> u32 {
    let mut index = 0;
    while char_matrix.len() > 1 {
        let filter_char = find_most_frequent(&char_matrix, index);
        char_matrix = only_keep_with_char_at(char_matrix, filter_char, index);
        index += 1;
    }
    binary_chars_to_decimal(&char_matrix.get(0).unwrap()[0..])
}

fn calc_co2(mut char_matrix: Vec<Vec<char>>) -> u32 {
    let mut index = 0;
    while char_matrix.len() > 1 {
        let filter_char = find_least_frequent(&char_matrix, index);
        char_matrix = only_keep_with_char_at(char_matrix, filter_char, index);
        index += 1;
    }
    binary_chars_to_decimal(&char_matrix.get(0).unwrap()[0..])
}

// oxygen -> if equal, return 1
fn find_most_frequent(char_matrix: &Vec<Vec<char>>, index: usize) -> char {
    let (nbr_of_zeroes, nbr_of_ones) = calc_ones_and_zeroes(char_matrix, index);
    if nbr_of_zeroes > nbr_of_ones {
        '0'
    } else {
        '1'
    }
}
// co2 -> if equal, return 0
fn find_least_frequent(char_matrix: &Vec<Vec<char>>, index: usize) -> char {
    let (nbr_of_zeroes, nbr_of_ones) = calc_ones_and_zeroes(char_matrix, index);
    if nbr_of_zeroes == 0 {
        return '1';
    } 
    if nbr_of_ones == 0 {
        return '0';
    }
    if nbr_of_zeroes <= nbr_of_ones {
        '0'
    } else {
        '1'
    }
}

fn calc_ones_and_zeroes(char_matrix: &Vec<Vec<char>>, index: usize) -> (usize, usize) {
    let nbr_of_zeroes = char_matrix
        .iter()
        .filter(|row| row.get(index).unwrap() == &'0')
        .count();
    let nbr_of_ones = char_matrix
        .iter()
        .filter(|row| row.get(index).unwrap() == &'1')
        .count();
    (nbr_of_zeroes, nbr_of_ones)
}

fn only_keep_with_char_at(char_matrix: Vec<Vec<char>>, ch: char, index: usize) -> Vec<Vec<char>> {
    char_matrix
        .into_iter()
        .filter(|row| row.get(index).unwrap() == &ch)
        .collect()
}

fn count(movements: Vec<Vec<char>>) -> u32 {
    let mut nbr_of_0s: Vec<u32> = Vec::new();
    let mut nbr_of_1s: Vec<u32> = Vec::new();

    for _ in 0..movements.get(0).unwrap().len() {
        nbr_of_0s.push(0);
        nbr_of_1s.push(0);
    }

    for row in movements {
        for (col, chr) in row.iter().enumerate() {
            match chr {
                '0' => {
                    *nbr_of_0s.get_mut(col).unwrap() += 1;
                }
                '1' => {
                    *nbr_of_1s.get_mut(col).unwrap() += 1;
                }
                _ => {}
            }
        }
    }

    let mut most_commons: Vec<u32> = Vec::new();
    let mut least_commons: Vec<u32> = Vec::new();
    for idx in 0..nbr_of_0s.len() {
        if nbr_of_0s.get(idx).unwrap() > nbr_of_1s.get(idx).unwrap() {
            most_commons.push(0);
            least_commons.push(1);
        } else {
            most_commons.push(1);
            least_commons.push(0);
        }
    }

    binary_to_decimal(most_commons) * binary_to_decimal(least_commons)
}

fn binary_chars_to_decimal(chars: &[char]) -> u32 {
    binary_to_decimal(chars.into_iter().map(|c| c.to_digit(10).unwrap()).collect())
}

fn binary_to_decimal(chars: Vec<u32>) -> u32 {
    let mut result = 0;

    let mut radix = 0;
    let base: u32 = 2;
    for digit in chars.iter().rev() {
        result += digit * base.pow(radix);
        radix += 1;
    }

    result.into()
}

fn line_to_chars(line: String) -> Vec<char> {
    line.chars().collect()
}

#[cfg(test)]
mod tests {

    use crate::day_3::{count, calc_oxygen, calc_co2};

    #[test]
    fn test_part_1() {
        let movements = vec![
            vec!['0', '1', '0'],
            vec!['0', '1', '1'],
            vec!['0', '0', '1'],
            vec!['1', '1', '1'],
            vec!['1', '0', '1'],
        ];
        // 0 1 1 -> 3
        // 1 0 0 -> 4
        assert_eq!(count(movements), 12);
    }

    #[test]
    fn test_part_2() {
        let movements = vec![
            vec!['0', '1', '0'],
            vec!['0', '1', '1'],
            vec!['0', '0', '1'],
            vec!['1', '1', '1'],
            vec!['1', '0', '1'],
        ];
        let movs2 = movements.clone();
        // MF 0 1 1 -> 3
        // LF 1 0 1 -> 5
        assert_eq!(calc_oxygen(movements), 3);
        assert_eq!(calc_co2(movs2), 5);
    }
}
