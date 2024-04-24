use crate::input::InputData;

mod year2021;
mod input;

fn main() {
    let result = year2021::day01::part_2(&InputData::new("input/year2021/day01".to_string()));
    println!("{}", result);
}
