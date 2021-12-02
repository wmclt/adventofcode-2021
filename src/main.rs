mod day_1;
mod day_2;
mod util;

fn main() {
    println!("Hello, world!");
    println!(
        "Answer day 1\n\tpart 1: {} \n\tpart 2: {}",
        day_1::part_1(),
        day_1::part_2()
    );
    println!(
        "Answer day 2\n\tpart 1: {} \n\tpart 2: {}",
        day_2::part_1(),
        day_2::part_2()
    );
}
