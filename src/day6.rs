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

    // Check the remaining characters
    for (i, b) in bytes[4..].iter().enumerate() {
        // Out with the previous character, in with the new one
        u = (u << 8) | (*b as u32);
        if all_bytes_different(u) {
            print_result(u, i + 5);
            return;
        }
    }

    unreachable!("Packet not found");
}

pub fn solve_part2() {
    const MESSAGE_SIZE: usize = 14;
    const ALPHA_SIZE: usize = 26;

    // Guaranteed ASCII
    let mut bytes = read_to_string("./input/day6input")
        .expect("Could not open input file")
        .into_bytes();

    // There is an unwanted newline at the end
    bytes.pop();

    // All the characters are lowercase letters. This remaps them into the range [0, ALPHA_SIZE)
    for b in bytes.iter_mut() {
        *b -= 97;
    }

    let mut counts = [0u8; ALPHA_SIZE];
    let mut num_duplicates = 0;
    let mut message = [0u8; MESSAGE_SIZE];

    // Initialize the counters with the first batch of characters
    message.copy_from_slice(&bytes[0..MESSAGE_SIZE]);
    for &b in message.iter() {
        let count = &mut counts[b as usize];
        *count += 1;
        if *count == 2 {
            num_duplicates += 1;
        }
    }

    // Already done?
    if num_duplicates == 0 {
        println!("Message found starting at 0");
        return;
    }

    // Check the remaining characters
    for (i, &b) in bytes[MESSAGE_SIZE..].iter().enumerate() {
        // We treat the message buffer as a ring buffer to avoid internal copying
        let ring_idx = i % MESSAGE_SIZE;

        // Decrement the character we're shifting out
        let shifted_out = message[ring_idx];
        let count = &mut counts[shifted_out as usize];
        *count -= 1;
        if *count == 1 {
            num_duplicates -= 1;
        }

        // Increment the character we're shifting in
        message[ring_idx] = b;
        let count = &mut counts[b as usize];
        *count += 1;
        if *count == 2 {
            num_duplicates += 1;
        }

        if num_duplicates == 0 {
            println!("Message found starting at {}", i + MESSAGE_SIZE + 1);
            return;
        }
    }

    unreachable!("Message not found");
}
