use std::cmp::Ordering;

use crate::helpers::iterate_file_lines;

#[derive(Debug)]
enum PacketValue {
    Int(usize),
    List(Vec<PacketValue>),
}

fn parse_packet_value(packet_str: &str) -> PacketValue {
    if packet_str.starts_with('[') {
        let inner = &packet_str[1..packet_str.len() - 1]; // Strip the outer brackets

        let list: Vec<PacketValue> = if inner.is_empty() {
            vec![]
        } else {
            // Make sure we don't hit commas of any nested lists
            let mut brace_level = 0;
            inner
                .split(|c| {
                    if c == '[' {
                        brace_level += 1;
                    } else if c == ']' {
                        brace_level -= 1;
                    }
                    brace_level == 0 && c == ','
                })
                .map(parse_packet_value)
                .collect()
        };

        PacketValue::List(list)
    } else {
        PacketValue::Int(packet_str.parse::<usize>().expect("Malformed int packet value"))
    }
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        // Int<->List comparison promotes the int to a 1-element list
        match (self, other) {
            (PacketValue::Int(left), PacketValue::Int(right)) => left.cmp(right),
            (PacketValue::Int(left), PacketValue::List(right)) => {
                // For some reason Rust refuses to do the comparison if the LHS is sized
                let smashed: &[PacketValue] = &[PacketValue::Int(*left)];
                smashed.cmp(right.as_slice())
            }
            (PacketValue::List(left), PacketValue::Int(right)) => {
                left.as_slice().cmp(&[PacketValue::Int(*right)])
            }
            (PacketValue::List(left), PacketValue::List(right)) => left.cmp(right),
        }
    }
}

// All unfortunate boilerplate to satisfy the requirements of Ord
impl Eq for PacketValue {}
impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for PacketValue {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

pub fn solve() {
    let mut sum = 0;
    let mut pair_idx = 1;
    let mut all_packets = Vec::new();

    let mut lines = iterate_file_lines("day13input.txt");
    loop {
        let Some(first_line) = lines.next() else {
            // EOF
            break;
        };
        let second_line = lines.next().expect("Unexpected EOF");

        let first_packet = parse_packet_value(&first_line);
        let second_packet = parse_packet_value(&second_line);

        if first_packet <= second_packet {
            sum += pair_idx;
        }
        all_packets.push(first_packet);
        all_packets.push(second_packet);

        lines.next(); // Eat a blank line
        pair_idx += 1;
    }

    println!("Sum is {sum}");

    all_packets.sort_unstable();

    // Indexing starts from one in packet land
    let first_divider = PacketValue::List(vec![PacketValue::Int(2)]);
    let first_divider_idx = all_packets.partition_point(|p| p < &first_divider) + 1;

    // The expectation is that the first divider is actually inserted into the packet list.
    // This would have the effect of bumping the index of all later packets, so the second divider
    // also needs one extra
    let second_divider = PacketValue::List(vec![PacketValue::Int(6)]);
    let second_divider_idx = all_packets.partition_point(|p| p < &second_divider) + 2;

    let decoder_key = first_divider_idx * second_divider_idx;
    println!("Decoder key is {decoder_key}");
}
