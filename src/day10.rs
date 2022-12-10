use crate::helpers::iterate_file_lines;

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

struct State {
    reg: i32,
    cycle: i32,
    sum: i32,
    screen: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl State {
    fn tick(&mut self) {
        self.cycle += 1;
        if (self.cycle - 20) % 40 == 0 {
            self.sum += self.cycle * self.reg;
        }
    }
}

pub fn solve() {
    let mut state = State {
        reg: 1,
        cycle: 0,
        sum: 0,
        screen: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
    };

    let mut last_was_add = false;

    for line in iterate_file_lines("day10input.txt") {
        state.tick();

        match line.as_str() {
            "noop" => {}
            s if s.starts_with("addx") && s.len() >= 6 => {
                let num = s[5..].parse::<i32>().expect("Malformed addx");

                state.tick();
                state.reg += num;
                last_was_add = true;
            }
            _ => panic!("Malformed input"),
        }
    }

    if last_was_add {
        state.tick();
    }

    println!("Sum is {}", state.sum);
}
