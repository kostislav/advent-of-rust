use crate::input::InputData;

mod year2021;
mod input;

macro_rules! run {
    ($year:ident, $day:ident, $part:ident) => {
        {
            let start_time = std::time::SystemTime::now();
            let input_data = InputData::new(format!("input/{}/{}", stringify!($year), stringify!($day)));
            let result = $year::$day::$part(&input_data);
            println!("Computation took {} ms", (start_time.elapsed().unwrap().as_micros() as f64) / 1000.0);
            result
        }
    };
}

fn main() {
    let result = run!(year2021, day01, part_2);
    println!("{}", result);
}