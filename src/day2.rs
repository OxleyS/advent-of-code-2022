use crate::helpers::iterate_file_lines;

#[derive(PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Won,
    Lost,
    Draw,
}

fn to_opponent_choice(c: char) -> Choice {
    match c {
        'A' => Choice::Rock,
        'B' => Choice::Paper,
        _ => Choice::Scissors,
    }
}

fn calc_points(my_choice: Choice, result: Result) -> usize {
    let me_points = match my_choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };
    let result_points = match result {
        Result::Won => 6,
        Result::Lost => 0,
        Result::Draw => 3,
    };
    me_points + result_points
}

pub fn solve_part1() {
    let mut total = 0usize;

    for line in iterate_file_lines("day2input") {
        fn to_my_choice(c: char) -> Choice {
            match c {
                'X' => Choice::Rock,
                'Y' => Choice::Paper,
                _ => Choice::Scissors,
            }
        }

        let mut chars = line.chars();
        let opp = to_opponent_choice(chars.next().expect("Malformed line"));
        chars.next();
        let my_choice = to_my_choice(chars.next().expect("Malformed line"));

        let result = match (&my_choice, opp) {
            (Choice::Rock, Choice::Rock) => Result::Draw,
            (Choice::Rock, Choice::Paper) => Result::Lost,
            (Choice::Rock, Choice::Scissors) => Result::Won,
            (Choice::Paper, Choice::Rock) => Result::Won,
            (Choice::Paper, Choice::Paper) => Result::Draw,
            (Choice::Paper, Choice::Scissors) => Result::Lost,
            (Choice::Scissors, Choice::Rock) => Result::Lost,
            (Choice::Scissors, Choice::Paper) => Result::Won,
            (Choice::Scissors, Choice::Scissors) => Result::Draw,
        };

        total += calc_points(my_choice, result);
    }

    println!("Total points {total}");
}

pub fn solve_part2() {
    let mut total = 0usize;

    for line in iterate_file_lines("day2input") {
        fn to_my_result(c: char) -> Result {
            match c {
                'X' => Result::Lost,
                'Y' => Result::Draw,
                _ => Result::Won,
            }
        }

        let mut chars = line.chars();
        let opp = to_opponent_choice(chars.next().expect("Malformed line"));
        chars.next();
        let result = to_my_result(chars.next().expect("Malformed line"));

        let my_choice = match (&result, &opp) {
            (Result::Won, Choice::Rock) => Choice::Paper,
            (Result::Won, Choice::Paper) => Choice::Scissors,
            (Result::Won, Choice::Scissors) => Choice::Rock,
            (Result::Draw, _) => opp,
            (Result::Lost, Choice::Rock) => Choice::Scissors,
            (Result::Lost, Choice::Paper) => Choice::Rock,
            (Result::Lost, Choice::Scissors) => Choice::Paper,
        };

        total += calc_points(my_choice, result);
    }

    println!("Total points {total}");
}
