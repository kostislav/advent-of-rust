use crate::input::InputData;

mod year2021;
mod input;

macro_rules! run {
    ($year:ident, $day:ident, $part:ident) => {
        $year::$day::$part(&InputData::new(format!("input/{}/{}", stringify!($year), stringify!($day))))
    };
}

fn main() {
    let result = run!(year2021, day01, part_2);
    println!("{}", result);
}
