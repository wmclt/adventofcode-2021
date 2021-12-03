use std::{collections::binary_heap, str::FromStr, vec::Vec};

use crate::util::read_data;

pub fn part_1() -> i32 {
    let file_path = "./data/day_3.txt";
    let instructions = read_data(file_path, line_to_chars);
    count(instructions)
}

pub fn part_2() -> i32 {
    let file_path = "./data/day_2.txt";
    let instructions = read_data(file_path, line_to_chars);
    // count_with_aim(instructions)
    6
}

fn count(movements: Vec<Vec<char>>) -> i32 {
    let mut nbr_of_0s: Vec<u16> = Vec::new();
    let mut nbr_of_1s: Vec<u16> = Vec::new();

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

    // println!("{:?}\n {:?}", nbr_of_0s, nbr_of_1s);

    let mut most_commons: Vec<u16> = Vec::new();
    let mut least_commons: Vec<u16> = Vec::new();
    for idx in 0..nbr_of_0s.len() {
        if nbr_of_0s.get(idx).unwrap() > nbr_of_1s.get(idx).unwrap() {
            most_commons.push(0);
            least_commons.push(1);
        } else {
            most_commons.push(1);
            least_commons.push(0);
        }
    }

    // println!("{:?}\n {:?}", most_commons, least_commons);

    binary_to_decimal(most_commons) * binary_to_decimal(least_commons)
}

fn binary_to_decimal(chars: Vec<u16>) -> i32 {
    let mut result = 0;

    let mut radix = 0;
    let base: u16 = 2;
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

    use crate::day_3::count;

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
}
