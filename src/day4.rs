use crate::helpers::iterate_file_lines;
use std::cmp::Ordering;

struct Range {
    start: usize,
    end: usize,
}

fn parse_range(s: &str) -> Range {
    let (a, b) = s.split_once('-').expect("Malformed pair");
    Range { start: a.parse().expect("Malformed A"), end: b.parse().expect("Malformed B") }
}

fn has_containment(a: Range, b: Range) -> bool {
    match a.start.cmp(&b.start) {
        Ordering::Greater => a.end <= b.end,
        Ordering::Less => b.end <= a.end,
        Ordering::Equal => true,
    }
}

fn has_overlap(a: Range, b: Range) -> bool {
    a.start <= b.end && a.end >= b.start
}

pub fn solve_part1() {
    let mut num_contained = 0;

    for line in iterate_file_lines("day4input.txt") {
        let (a_str, b_str) = line.split_once(',').expect("Malformed line");
        let (a_range, b_range) = (parse_range(a_str), parse_range(b_str));

        if has_containment(a_range, b_range) {
            num_contained += 1;
        }
    }

    println!("Total is {num_contained}");
}

pub fn solve_part2() {
    let mut num_contained = 0;

    for line in iterate_file_lines("day4input.txt") {
        let (a_str, b_str) = line.split_once(',').expect("Malformed line");
        let (a_range, b_range) = (parse_range(a_str), parse_range(b_str));

        if has_overlap(a_range, b_range) {
            num_contained += 1;
        }
    }

    println!("Total is {num_contained}");
}
