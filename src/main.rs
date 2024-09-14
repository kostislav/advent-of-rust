mod year2021;
mod input;
mod array;
mod benchmark;
mod collections;
mod graph;

#[allow(unused_macros)]
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
    // let result = run!(year2021, day17, part_2);
    // println!("{}", result);
    benchmark_all!(
        day01,
        day02,
        day03,
        day04,
        day05,
        day06,
        day07,
        day08,
        day09,
        day10,
        day11,
        day12,
        day13,
        day14,
        day15,
        day16,
        day17,
        day18,
        day19,
        day20
    );
}
