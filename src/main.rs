mod year2021;
mod input;
mod array;

macro_rules! run {
    ($year:ident, $day:ident, $part:ident) => {
        {
            let input_data = crate::input::InputData::from_file(&format!("input/{}/{}", stringify!($year), stringify!($day)));
            let start_time = std::time::SystemTime::now();
            let result = $year::$day::$part(&input_data);
            println!("Computation took {} μs", start_time.elapsed().unwrap().as_micros() as f64);
            result
        }
    };
}

fn main() {
    let result = run!(year2021, day09, part_1);
    println!("{}", result);
}
