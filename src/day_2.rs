use std::{str::FromStr, vec::Vec};

use crate::util::read_data;

pub fn part_1() -> i32 {
    let file_path = "./data/day_2.txt";
    let instructions = read_data(file_path, line_to_movement);
    count(instructions)
}

pub fn part_2() -> i32 {
    let file_path = "./data/day_2.txt";
    let instructions = read_data(file_path, line_to_movement);
    count_with_aim(instructions)
}

fn count(movements: Vec<(String, i32)>) -> i32 {
    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;

    for (dir, nbr) in movements {
        match dir.as_str() {
            "forward" => {
                horizontal += nbr;
            }
            "down" => {
                vertical += nbr;
            }
            "up" => {
                vertical -= nbr;
            }
            _ => {}
        }
    }

    horizontal * vertical
}

fn count_with_aim(movements: Vec<(String, i32)>) -> i32 {
    let mut aim: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for (dir, nbr) in movements {
        match dir.as_str() {
            "forward" => {
                horizontal += nbr;
                depth += aim * nbr;
            }
            "down" => {
                aim += nbr;
            }
            "up" => {
                aim -= nbr;
            }
            _ => {}
        }
    }

    horizontal * depth
}

fn line_to_movement(line: String) -> (String, i32) {
    let mut split_string: Vec<&str> = line.split(' ').collect();
    let nbr: i32 = FromStr::from_str(split_string.pop().unwrap()).unwrap();
    let direction = split_string.pop().unwrap().to_string();
    (direction, nbr)
}

#[cfg(test)]
mod tests {

    use crate::day_2::count;

    #[test]
    fn test_part_1() {
        let movements = vec![
            ("forward".to_string(), 5),
            ("down".to_string(), 5),
            ("forward".to_string(), 5),
            ("up".to_string(), 2),
            ("down".to_string(), 6),
        ];
        assert_eq!(count(movements), 90);
    }
}
