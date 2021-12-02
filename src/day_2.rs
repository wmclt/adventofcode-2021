use std::{str::FromStr, vec::Vec};

pub fn part_1() -> i32 {
    let file_path = "./data/day_2.txt";
    let instructions = load_instructions(file_path);
    count(instructions)
}

pub fn part_2() -> i32 {
    let file_path = "./data/day_2.txt";
    let instructions = load_instructions(file_path);
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

fn load_instructions(filename: &str) -> Vec<(String, i32)> {
    let mut direction_moves: Vec<(String, i32)> = Vec::new();
    if let Ok(lines) = crate::util::read_lines(filename) {
        for line in lines.flatten() {
            let (nbr, direction) = line_to_movement(line);
            direction_moves.push((direction, nbr));
        }
    }
    direction_moves
}

fn line_to_movement(line: String) -> (i32, String) {
    let mut split_string: Vec<&str> = line.split(' ').collect();
    let nbr: i32 = FromStr::from_str(split_string.pop().unwrap()).unwrap();
    let direction = split_string.pop().unwrap().to_string();
    (nbr, direction)
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
