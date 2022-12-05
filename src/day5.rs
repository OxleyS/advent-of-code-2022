use crate::helpers::iterate_file_lines;

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
        iter.nth(1)
            .and_then(|s| s.parse::<usize>().ok())
            .expect("Malformed move line")
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

fn double_index(crate_stacks: &mut StackSet, idx1: usize, idx2: usize) -> (&mut Stack, &mut Stack) {
    // SAFETY: These two references do not alias as long as the two indices are different
    assert_ne!(idx1, idx2);
    unsafe {
        (
            &mut *(&mut crate_stacks[idx1] as *mut Stack),
            &mut *(&mut crate_stacks[idx2] as *mut Stack),
        )
    }
}

pub fn solve_part1() {
    let mut lines = iterate_file_lines("day5input");
    let mut crate_stacks = parse_crate_stacks(&mut lines);

    lines.next(); // Skip a line
    for line in lines {
        let mv = parse_move_line(&line);

        // Unnecessary, but more fun than a pop-push loop!
        let (src, dest) = double_index(&mut crate_stacks, mv.src_stack, mv.dest_stack);
        let src_iter = src.drain((src.len() - mv.number)..).rev();
        dest.extend(src_iter);
    }

    let message = collect_message(&crate_stacks);
    println!("The message is {message}");
}

pub fn solve_part2() {
    let mut lines = iterate_file_lines("day5input");
    let mut crate_stacks = parse_crate_stacks(&mut lines);

    lines.next(); // Skip a line
    for line in lines {
        let mv = parse_move_line(&line);

        // Unnecessary, but more fun than a pop-push loop!
        let (src, dest) = double_index(&mut crate_stacks, mv.src_stack, mv.dest_stack);
        let src_iter = src.drain((src.len() - mv.number)..);
        dest.extend(src_iter);
    }

    let message = collect_message(&crate_stacks);
    println!("The message is {message}");
}
