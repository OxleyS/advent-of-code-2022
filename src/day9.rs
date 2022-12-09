use std::collections::HashSet;

use crate::helpers::iterate_file_lines;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

pub fn solve() {
    let mut head = Coord { x: 0, y: 0 };
    let mut tail = head;

    let mut visited: HashSet<Coord> = HashSet::new();
    visited.insert(tail);

    for line in iterate_file_lines("day9input.txt") {
        let (direction, num_steps) = line.split_once(' ').expect("Malformed line");
        let num_steps = num_steps.parse::<usize>().expect("Malformed step count");

        // TODO: Optimize this to be branchless?
        let move_amt = match direction {
            "L" => (-1i32, 0i32),
            "R" => (1i32, 0i32),
            "U" => (0i32, -1i32),
            "D" => (0i32, 1i32),
            _ => panic!("Malformed direction"),
        };

        for _ in 0..num_steps {
            head.x += move_amt.0;
            head.y += move_amt.1;

            let delta = Coord {
                x: head.x - tail.x,
                y: head.y - tail.y,
            };

            // No need to move?
            if delta.x.abs() < 2 && delta.y.abs() < 2 {
                continue;
            }

            tail.x += delta.x.signum();
            tail.y += delta.y.signum();

            visited.insert(tail);
        }
    }

    println!("Places visited: {}", visited.len());
}
