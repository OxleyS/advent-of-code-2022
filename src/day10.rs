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
    fn new() -> Self {
        Self { reg: 1, cycle: 0, sum: 0, screen: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT] }
    }

    fn do_noop(&mut self) {
        self.tick();
    }

    fn do_addx(&mut self, operand: i32) {
        self.tick();
        self.tick();
        self.reg += operand;
    }

    fn tick(&mut self) {
        self.cycle += 1;
        if (self.cycle - 20) % 40 == 0 {
            self.sum += self.cycle * self.reg;
        }

        let draw_cycle = (self.cycle - 1) % (SCREEN_WIDTH * SCREEN_HEIGHT) as i32;
        let y_pos = draw_cycle / (SCREEN_WIDTH as i32);
        let x_pos = draw_cycle - (y_pos * (SCREEN_WIDTH as i32));

        let pixel_lit = (x_pos - self.reg).abs() <= 1;
        self.screen[y_pos as usize][x_pos as usize] = pixel_lit;
    }
}

pub fn solve() {
    let mut state = State::new();

    for line in iterate_file_lines("day10input.txt") {
        match line.as_str() {
            "noop" => state.do_noop(),
            s if s.starts_with("addx") && s.len() >= 6 => {
                let num = s[5..].parse::<i32>().expect("Malformed addx");
                state.do_addx(num);
            }
            _ => panic!("Malformed input"),
        }
    }

    println!("Sum is {}", state.sum);
    println!("Screen contents:");
    let screen_lines = state
        .screen
        .iter()
        .map(|row| row.iter().map(|&lit| if lit { '#' } else { '.' }).collect::<String>());
    for line in screen_lines {
        println!("{line}");
    }
}
