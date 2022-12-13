use std::fs::read_to_string;

use crate::helpers::iterate_file_lines;

pub fn solve() {
    solve_part1();
    solve_part2();
}

pub fn solve_short() {
    println!("Moved one at a time: {}", solve_short_impl(false));
    println!("Moved multiple at a time: {}", solve_short_impl(true));
}

const NUM_STACKS: usize = 9;

type Stack = Vec<u8>;
type StackSet = [Stack; NUM_STACKS];

struct Move {
    number: usize,
    src_stack: usize,
    dest_stack: usize,
}

// Takes any iterator whose item can be taken as a &str
fn parse_crate_stacks<S: AsRef<str>>(lines: &mut impl Iterator<Item = S>) -> StackSet {
    const EMPTY_VEC: Stack = Vec::new();
    let mut stacks = [EMPTY_VEC; NUM_STACKS];
    loop {
        // Explicitly bind the item returned by the iterator so that it lives long enough
        let orig_type_line = lines.next().expect("Unexpected input end");

        let line = orig_type_line.as_ref();
        if line.starts_with(" 1") {
            for stack in &mut stacks {
                stack.reverse();
            }
            break stacks;
        }

        let chars = &line.as_bytes()[1..];
        for (i, stack) in stacks.iter_mut().enumerate() {
            let c = *chars.get(i * 4).expect("Malformed crate line");
            if (c as char) != ' ' {
                stack.push(c);
            }
        }
    }
}

fn parse_move_line(line: &str) -> Move {
    fn parse_one(iter: &mut std::str::SplitN<char>) -> usize {
        iter.nth(1).and_then(|s| s.parse::<usize>().ok()).expect("Malformed move line")
    }

    let mut substring_iter = line.splitn(6, ' ');
    Move {
        number: parse_one(&mut substring_iter),
        src_stack: parse_one(&mut substring_iter) - 1,
        dest_stack: parse_one(&mut substring_iter) - 1,
    }
}

fn collect_message(crate_stacks: &StackSet) -> String {
    crate_stacks
        .iter()
        .map(|stack| stack.last().map(|&c| c as char).unwrap_or(' '))
        .collect::<String>()
}

fn solve_part1() {
    let mut lines = iterate_file_lines("day05input.txt");
    let mut crate_stacks = parse_crate_stacks(&mut lines);

    lines.next(); // Skip a line
    for line in lines {
        let mv = parse_move_line(&line);
        let [src, dest] =
            crate_stacks.get_many_mut([mv.src_stack, mv.dest_stack]).expect("Bad stack indices");
        let src_iter = src.drain((src.len() - mv.number)..).rev();
        dest.extend(src_iter);
    }

    let message = collect_message(&crate_stacks);
    println!("The message is {message}");
}

fn solve_part2() {
    let mut lines = iterate_file_lines("day05input.txt");
    let mut crate_stacks = parse_crate_stacks(&mut lines);

    lines.next(); // Skip a line
    for line in lines {
        let mv = parse_move_line(&line);
        let [src, dest] =
            crate_stacks.get_many_mut([mv.src_stack, mv.dest_stack]).expect("Bad stack indices");
        let src_iter = src.drain((src.len() - mv.number)..);
        dest.extend(src_iter);
    }

    let message = collect_message(&crate_stacks);
    println!("The message is {message}");
}

fn solve_short_impl(multi_move: bool) -> String {
    const EMPTY_VEC: Stack = Vec::new();
    let input = read_to_string("./input/day05input.txt").expect("Could not read input");
    let (crate_layout, commands) = input.split_once("\n\n").expect("Could not split input");

    let mut crate_stacks = [EMPTY_VEC; NUM_STACKS];
    for line in crate_layout.lines().map(|l| l.chars().collect::<Vec<_>>()) {
        for (i, chunk) in line.chunks(4).enumerate().filter(|(_, s)| s[0] == '[') {
            crate_stacks[i].insert(0, chunk[1] as u8);
        }
    }

    for line in commands.lines() {
        let [number, from, to] = line
            .splitn(6, ' ')
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<usize>().expect("Could not parse number"))
            .collect::<Vec<_>>()[..] else {
                panic!("Expected 3-tuple");
            };

        let from_stack = &mut crate_stacks[from - 1];
        let mut crates = from_stack.drain((from_stack.len() - number)..).collect::<Vec<_>>();
        if !multi_move {
            crates.reverse();
        }
        crate_stacks[to - 1].extend(crates.into_iter());
    }

    collect_message(&crate_stacks)
}
