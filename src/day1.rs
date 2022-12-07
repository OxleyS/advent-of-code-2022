use crate::helpers::iterate_file_lines;

pub fn solve_part1() {
    let mut largest: usize = 0;
    let mut cur_total: usize = 0;

    for line in iterate_file_lines("day1input.txt") {
        if line.is_empty() {
            largest = largest.max(cur_total);
            cur_total = 0;
        } else {
            let calories: usize = line.parse().expect("Expected parseable number");
            cur_total += calories;
        }
    }

    println!("Largest calorie total is {largest}");
}

pub fn solve_part2() {
    let mut largest_three = [0usize; 4]; // One extra to avoid bounds checks in copy_within
    let mut cur_total: usize = 0;

    for line in iterate_file_lines("day1input.txt") {
        if line.is_empty() {
            let insert_index = largest_three[0..3].partition_point(|&x| x > cur_total);
            if insert_index < 3 {
                largest_three.copy_within(insert_index..2, insert_index + 1);
                largest_three[insert_index] = cur_total;
            }
            cur_total = 0;
        } else {
            let calories: usize = line.parse().expect("Expected parseable number");
            cur_total += calories;
        }
    }

    println!(
        "Largest calorie total is {}",
        largest_three[0..3].iter().sum::<usize>()
    );
}
