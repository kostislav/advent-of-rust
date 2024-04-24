use crate::input::InputData;
use crate::year2021::day01;

mod year2021;
mod input;

fn main() {
    let result = day01::part_1(&InputData::new("input/2021/day01".to_string()));
    println!("{}", result);
}
