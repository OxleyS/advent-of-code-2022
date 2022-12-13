use std::cmp::Ordering;

use crate::helpers::iterate_file_lines;

pub fn solve() {
    let mut all_packets: Vec<PacketValue> = iterate_file_lines("day13input.txt")
        .filter(|line| !line.is_empty())
        .map(|line| parse_packet_value(&line))
        .collect();

    let mut sum = 0;
    for (i, [left, right]) in all_packets.iter().array_chunks().enumerate() {
        if left <= right {
            // Indexing starts from one in packet land
            sum += i + 1;
        }
    }
    println!("Sum of in-order pair indices is {sum}");

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

#[derive(Debug)]
enum PacketValue {
    Int(usize),
    List(Vec<PacketValue>),
}

fn parse_packet_value(packet_str: &str) -> PacketValue {
    match packet_str {
        "[]" => PacketValue::List(vec![]),
        packet_str if packet_str.starts_with('[') => {
            // Strip the outer brackets
            let inner = &packet_str[1..packet_str.len() - 1];

            // Make sure we don't hit commas of any nested lists
            let mut brace_level = 0;
            let list = inner
                .split(|c| {
                    if c == '[' {
                        brace_level += 1;
                    } else if c == ']' {
                        brace_level -= 1;
                    }
                    brace_level == 0 && c == ','
                })
                .map(parse_packet_value)
                .collect();

            PacketValue::List(list)
        }
        _ => PacketValue::Int(packet_str.parse::<usize>().expect("Malformed int packet value")),
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
