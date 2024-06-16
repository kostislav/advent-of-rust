use std::path::Path;
use crate::input::InputData;

const W_DAY: usize = 10;
const W_PART: usize = 10;

fn mean(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

fn measure_run<S: ?Sized, T, F: Fn(&S) -> T>(f: &F, input: &S) -> f64 {
    let start = std::time::SystemTime::now();
    _ = f(input);
    let duration = start.elapsed().unwrap();
    duration.as_secs_f64()
}

pub fn benchmark_run<S: ?Sized, T, F: Fn(&S) -> T>(f: F, input: &S) -> f64 {
    let first_run = measure_run(&f, input);
    let n = (1. / first_run) as i32;
    if n <= 1 || first_run < 0.000001 {
        return first_run;
    }
    let mut run_times = vec![];
    for _ in 0..n {
        run_times.push(measure_run(&f, input));
    }
    mean(&run_times)
}

pub fn print_header() {
    print!("{:<w$}", "day", w = W_DAY);
    print!("{:<w$}", "part 1", w = W_PART);
    print!("{:<w$}", "part 2", w = W_PART);
    println!();
    println!("{:-<w$}", "", w = W_DAY + W_PART * 2);
}

pub fn print_day(day: u8, p1: f64, p2: f64) {
    print!("{:<w$}", format!("day {:02}", day), w = W_DAY);

    let mut p1_dur = format!("{:.3}", p1 * 1000.).to_string();
    p1_dur = format!("{} ms", &p1_dur[..5]);
    print!("{:<w$}", p1_dur, w = W_PART);

    let mut p2_dur = format!("{:.3}", p2 * 1000.).to_string();
    p2_dur = format!("{} ms", &p2_dur[..5]);
    println!("{:<w$}", p2_dur, w = W_PART);
}

#[macro_export]
macro_rules! benchmark_all {
    ($($day:ident),*) => {{
        $crate::benchmark::print_header();
        $(
        let input = $crate::benchmark::find_input(&stringify!($day));

        let p1_duration = $crate::benchmark::benchmark_run($crate::year2021::$day::part_1, &input);
        let p2_duration = $crate::benchmark::benchmark_run($crate::year2021::$day::part_2, &input);

        $crate::benchmark::print_day(stringify!($day).to_string()[3..].parse().unwrap(), p1_duration, p2_duration);
        )*
    }};
}

pub fn find_input(day: &str) -> InputData {
    let file_name = format!("input/year2021/{}", day);
    let regular_location = Path::new(file_name.as_str());
    if regular_location.exists() {
        InputData::from_file(regular_location.to_str().unwrap())
    } else {
        let file_name = format!("inputs/{}.in", &day[3..]);
        InputData::from_file(file_name.as_str())
    }
}
