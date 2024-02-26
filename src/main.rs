use std::fs;
use day_runner::DayRunner;

mod day_runner;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let day = day7::Day7{}; 
    run(&day, 7);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn run(runner: &dyn DayRunner, day_number: u32) {

    let file = fs::read_to_string(format!("./inputs/day{}.txt", day_number)).unwrap();
    let lines: Vec<String> = file.lines().map(|s| s.to_string()).collect();
    
    runner.run_p2(lines);
}