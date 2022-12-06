use std::fs::read_to_string;

pub fn solve_part1() {
    fn has_zero_byte(u: u32) -> bool {
        // From https://graphics.stanford.edu/~seander/bithacks.html#ZeroInWord
        ((u.wrapping_sub(0x01010101u32)) & !u & 0x80808080u32) != 0
    }

    fn all_bytes_different(u: u32) -> bool {
        // We need to test each byte with each other byte. A fast way to do this is to XOR the bytes
        // against each other, and check for zero bytes (meaning there was a byte match).
        // There are six pairs to test, with one XOR we can test at most four pairs, thus we need
        // two XORs to get them all
        let r1 = u.rotate_right(8);
        let r2 = u.rotate_right(16);
        !has_zero_byte(u ^ r1) && !has_zero_byte(u ^ r2)
    }

    fn print_result(u: u32, idx: usize) {
        let s: String = u.to_ne_bytes().iter().map(|&c| c as char).collect();
        println!("Found {s} at index {idx}");
    }

    // Guaranteed ASCII
    let bytes = read_to_string("./input/day6input")
        .expect("Could not open input file")
        .into_bytes();

    // We pack each of the four characters we test into a u32, for quick testing.
    // Special-case check the first four, for loop simplicity
    let mut u = u32::from_ne_bytes(bytes[0..4].try_into().expect("Input not long enough"));
    if all_bytes_different(u) {
        print_result(u, 0);
        return;
    }

    // Check the remaining bytes
    for (i, b) in bytes[4..].iter().enumerate() {
        // Out with the previous character, in with the new one
        u = (u << 8) | (*b as u32);
        if all_bytes_different(u) {
            print_result(u, i + 5);
            return;
        }
    }
}
