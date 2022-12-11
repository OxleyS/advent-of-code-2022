use crate::helpers::iterate_file_lines;

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    divis_test: usize,
    divis_target: usize,
    non_divis_target: usize,
    inspect_count: usize,
}

fn extract_line<T>(
    lines: &mut impl Iterator<Item = String>,
    prefix: &str,
    f: impl FnOnce(&str) -> T,
) -> T {
    let line = lines.next().expect("Unexpected EOF");
    f(line
        .trim_start()
        .strip_prefix(prefix)
        .expect("Expected prefix"))
}

fn parse_monkey(lines: &mut impl Iterator<Item = String>) -> Monkey {
    let items: Vec<usize> = extract_line(lines, "Starting items: ", |data| {
        data.split(", ")
            .map(|s| s.parse::<usize>().expect("Malformed item list"))
            .collect()
    });

    let op: Operation = extract_line(lines, "Operation: new = old ", |data| {
        let (operator, operand) = data.split_once(' ').expect("Malformed op");
        match operator {
            "+" => Operation::Add(operand.parse::<usize>().expect("Malformed op")),
            "*" if operand == "old" => Operation::Square,
            "*" => Operation::Multiply(operand.parse::<usize>().expect("Malformed op")),
            _ => panic!("Malformed op"),
        }
    });

    let divis_test = extract_line(lines, "Test: divisible by ", |data| {
        data.parse::<usize>().expect("Malformed divisibility test")
    });

    let divis_target = extract_line(lines, "If true: throw to monkey ", |data| {
        data.parse::<usize>().expect("Malformed target monkey")
    });

    let non_divis_target = extract_line(lines, "If false: throw to monkey ", |data| {
        data.parse::<usize>().expect("Malformed target monkey")
    });

    Monkey {
        items,
        op,
        divis_test,
        divis_target,
        non_divis_target,
        inspect_count: 0,
    }
}

fn solve_impl(rounds: usize, worry_decay_factor: usize) -> usize {
    let mut lines = iterate_file_lines("day11input.txt");
    let mut monkeys: Vec<Monkey> = Vec::new();

    loop {
        // "Monkey N:" or EOF
        if lines.next().is_none() {
            break;
        };

        monkeys.push(parse_monkey(&mut lines));
        lines.next(); // Eat a blank line
    }

    let common_multiple: usize = monkeys.iter().map(|m| m.divis_test).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let (divis_idx, non_divis_idx) = (monkeys[i].divis_target, monkeys[i].non_divis_target);
            let [monkey, divis_target, non_divis_target] = monkeys
                .get_many_mut([i, divis_idx, non_divis_idx])
                .expect("Monkey throws to itself");

            monkey.inspect_count += monkey.items.len();
            for item in monkey.items.drain(..) {
                let undecayed_worry = match monkey.op {
                    Operation::Add(n) => item + n,
                    Operation::Multiply(n) => item * n,
                    Operation::Square => item * item,
                };

                let worry = (undecayed_worry / worry_decay_factor) % common_multiple;
                let divisible = worry % monkey.divis_test == 0;
                if divisible {
                    divis_target.items.push(worry);
                } else {
                    non_divis_target.items.push(worry);
                }
            }
        }
    }

    let mut top_counts: Vec<usize> = monkeys.iter().map(|m| m.inspect_count).collect();
    top_counts.sort_unstable();
    top_counts.reverse();

    top_counts[0] * top_counts[1]
}

pub fn solve() {
    println!(
        "Amount of monkey business (with decay, 20 rounds): {}",
        solve_impl(20, 3),
    );
    println!(
        "Amount of monkey business (without decay, 10000 rounds): {}",
        solve_impl(10000, 1),
    );
}
