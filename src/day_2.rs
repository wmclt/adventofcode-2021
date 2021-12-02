use std::{str::FromStr, vec::Vec};

pub fn part_1() -> i32 {
    let file_path = "./data/day_2.txt";
    let instructions = load_instructions(file_path);
    count(instructions)
}

pub fn part_2() -> u32 {
    666
    // let file_path = "./data/day_1.txt";
    // let numbers = load_numbers(file_path);
    // let mut window_numbers = sum_windows(numbers);
    // count_increases(&mut window_numbers)
}

// fn sum_windows(numbers: Vec<i32>) -> Vec<i32> {
//     let mut window_sums = Vec::new();
//     for i in 0..numbers.len() - 2 {
//         window_sums.push(numbers[i] + numbers[i + 1] + numbers[i + 2]);
//     }
//     window_sums
// }

fn count(movements: Vec<(String, i32)>) -> i32 {
    let mut horizontal:i32 = 0;
    let mut vertical:i32 = 0;

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

    (horizontal * vertical).into()
}

fn load_instructions(filename: &str) -> Vec<(String, i32)> {
    let mut direction_moves: Vec<(String, i32)> = Vec::new();
    if let Ok(lines) = crate::util::read_lines(filename) {
        for line in lines.flatten() {
            let mut split_string: Vec<&str> = line.split(" ").collect();
            let nbr: i32 = FromStr::from_str(split_string.pop().unwrap()).unwrap();
            let direction = split_string.pop().unwrap().to_string();
            direction_moves.push((direction, nbr));
        }
    }
    direction_moves
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

    // #[test]
    // fn other_test_part_1() {
    //     let mut numbers = vec![32, 21, 25, 226, 321, 488, 531];
    //     assert_eq!(count_increases(&mut numbers), 5);
    // }

    // #[test]
    // fn test_sum_windows() {
    //     let numbers = vec![1, 2, 3, 4, 6, 4, 4, 4];
    //     assert_eq!(sum_windows(numbers), vec![6, 9, 13, 14, 14, 12]);
    // }

    // #[test]
    // fn test_part_2() {
    //     let numbers = vec![1, 2, 3, 4, 6, 4, 4, 4];
    //     assert_eq!(count_increases(&mut sum_windows(numbers)), 3);
    // }
}
