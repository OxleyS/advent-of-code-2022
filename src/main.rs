#![feature(iter_array_chunks)]
#![feature(get_many_mut)]
#![feature(array_windows)]
#![feature(result_option_inspect)]
#![feature(let_chains)]
#![allow(dead_code)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod helpers;

const DAY_SOLVERS: &[fn()] = &[
    day01::solve,
    day02::solve,
    day03::solve,
    day04::solve,
    day05::solve,
    day06::solve,
    day07::solve,
    day08::solve,
    day09::solve,
    day10::solve,
    day11::solve,
    day12::solve,
    day13::solve,
];

fn print_usage_and_exit(program_name: &str) -> ! {
    let file_name = std::path::Path::new(program_name)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("program_name");

    eprintln!("USAGE: {file_name} [day_number (1-{})]", DAY_SOLVERS.len());
    eprintln!("If omitted, the day number defaults to the latest day.");
    eprintln!("EXAMPLE: \"{file_name} 2\" solves Day 2.");
    std::process::exit(1);
}

fn fatal_error(s: &str) -> ! {
    eprintln!("Error: {s}");
    std::process::exit(1);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let day_number: usize = match args.as_slice() {
        [name, _, _, ..] => print_usage_and_exit(name),
        [name, s] => {
            if s == "--usage" || s == "--help" {
                print_usage_and_exit(name);
            }

            let n = s.parse::<isize>().unwrap_or_else(|_| fatal_error("Expected a day number"));
            if n < 1 || (n as usize) > DAY_SOLVERS.len() {
                fatal_error("Invalid day number")
            }
            n as usize
        }
        _ => DAY_SOLVERS.len(),
    };

    println!("--- Solving Day {day_number} ---");
    DAY_SOLVERS[day_number - 1]();
}
