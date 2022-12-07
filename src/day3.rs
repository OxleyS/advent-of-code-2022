use crate::helpers::iterate_file_lines;

fn get_priority(c: u8) -> usize {
    (match c {
        c if c >= 97 => c - 96, // Lowercase
        c => c - 38,            // Uppercase
    }) as usize
}

pub fn solve_part1() {
    let mut total = 0usize;

    for line in iterate_file_lines("day3input.txt") {
        // We know it's ASCII
        let half_point = line.len() / 2;
        let (first_comp, second_comp) =
            (line[..half_point].as_bytes(), line[half_point..].as_bytes());

        // We know there's exactly one match
        let common = *first_comp
            .iter()
            .find(|b| second_comp.contains(b))
            .expect("Should have been a match");

        total += get_priority(common);
    }

    println!("Total is {total}");
}

pub fn solve_part2() {
    let mut total = 0usize;

    for [l1, l2, l3] in iterate_file_lines("day3input.txt").array_chunks::<3>() {
        // We know it's ASCII
        let (b1, b2, b3) = (l1.as_bytes(), l2.as_bytes(), l3.as_bytes());

        // We know there's exactly one match
        let common = *b1
            .iter()
            .find(|b| b2.contains(b) && b3.contains(b))
            .expect("Should have been a match");

        total += get_priority(common);
    }

    println!("Total is {total}");
}
