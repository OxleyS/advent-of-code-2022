use std::collections::HashSet;

use crate::helpers::iterate_file_lines;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

fn solve_impl<const N: usize>() -> usize {
    let mut knots = [Coord { x: 0, y: 0 }; N];
    let mut visited: HashSet<Coord> = HashSet::new();
    visited.insert(knots[N - 1]); // Be sure to include initial position!

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
            knots[0].x += move_amt.0;
            knots[0].y += move_amt.1;

            for i in 0..(N - 1) {
                let [front, back] = knots.get_many_mut([i, i + 1]).unwrap();

                let delta = Coord {
                    x: front.x - back.x,
                    y: front.y - back.y,
                };

                // No need to move?
                if delta.x.abs() < 2 && delta.y.abs() < 2 {
                    continue;
                }

                back.x += delta.x.signum();
                back.y += delta.y.signum();
            }

            visited.insert(knots[N - 1]);
        }
    }

    visited.len()
}

pub fn solve() {
    println!("Visited by 2 knots: {}", solve_impl::<2>());
    println!("Visited by 10 knots: {}", solve_impl::<10>());
}
