use crate::helpers::iterate_file_lines;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Sensor {
    pos: Coord,
    manhattan: i32,
}

pub fn solve() {
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    for line in iterate_file_lines("day15input.txt") {
        let (sensor_str, beacon_str) = line.split_once(": ").expect("Malformed line");

        let sensor_start = "Sensor at ";
        assert!(sensor_str.starts_with(sensor_start));
        let sensor_coord_str = &sensor_str[sensor_start.len()..];

        let beacon_start = "closest beacon is at ";
        assert!(beacon_str.starts_with(beacon_start));
        let beacon_coord_str = &beacon_str[beacon_start.len()..];

        let sensor_coord = parse_coord(sensor_coord_str);
        let beacon_coord = parse_coord(beacon_coord_str);
        let manhattan = manhattan_distance(&sensor_coord, &beacon_coord);

        if !beacons.contains(&beacon_coord) {
            beacons.push(beacon_coord)
        }
        sensors.push(Sensor { pos: sensor_coord, manhattan });
    }

    let part1_row = 2000000; // Test: 10, Actual: 2000000
    let num_positions = solve_part1(part1_row, &sensors, &beacons);
    println!("{num_positions} positions cannot contain a beacon on row {part1_row}");

    let part2_range = 0..4000001; // Test: 21, Actual: 4000001
    let tuning_freq = solve_part2(&sensors, part2_range).expect("No position found");
    println!("Tuning frequency: {tuning_freq}");
}

fn parse_coord(s: &str) -> Coord {
    let (x_equals, y_equals) = s.split_once(", ").expect("Malformed coord");
    Coord {
        x: x_equals[2..].parse::<i32>().expect("Malformed X"),
        y: y_equals[2..].parse::<i32>().expect("Malformed Y"),
    }
}

fn manhattan_distance(a: &Coord, b: &Coord) -> i32 {
    (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)) as i32
}

fn solve_part1(y: i32, sensors: &[Sensor], beacons: &[Coord]) -> usize {
    let mut ranges: Vec<std::ops::Range<i32>> = Vec::new();

    // Calculate the ranges that sensors extend onto this row
    for sensor in sensors {
        let from_row: i32 = sensor.manhattan - (y.abs_diff(sensor.pos.y) as i32);
        if from_row >= 0 {
            ranges.push((sensor.pos.x - from_row)..(sensor.pos.x + from_row + 1));
        }
    }

    // Collapse the ranges into a count, taking care of overlap between ranges
    ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));
    let (total, _) = ranges.iter().fold((0usize, i32::MIN), |(total, last_x_end), x_range| {
        let disjoint_range = x_range.start.max(last_x_end)..x_range.end.max(last_x_end);
        let disjoint_size = (disjoint_range.end - disjoint_range.start) as usize;
        (total + disjoint_size, disjoint_range.end)
    });

    // Find the beacons that are actually on this row - they don't count as "cannot have beacon"
    let row_beacons = beacons
        .iter()
        .filter(|beacon| beacon.y == y && ranges.iter().any(|r| r.contains(&beacon.x)))
        .count();

    total - row_beacons
}

fn solve_part2(sensors: &[Sensor], full_range: std::ops::Range<i32>) -> Option<isize> {
    fn calc_tuning_freq(x: i32, y: i32) -> isize {
        ((x as isize) * 4000000) + (y as isize)
    }

    // Sort the sensors by X to maximize the chance that the ranges are already sorted
    let mut sensors = sensors.to_vec();
    sensors.sort_by(|a, b| a.pos.x.cmp(&b.pos.x));

    // This Vec is reused to avoid per-iteration allocation
    let mut ranges: Vec<std::ops::Range<i32>> = Vec::with_capacity(sensors.len());

    let expected_end = full_range.end;
    for y in full_range {
        // Calculate the ranges that sensors extend onto this row
        for sensor in sensors.iter() {
            let from_row: i32 = sensor.manhattan - (y.abs_diff(sensor.pos.y) as i32);
            if from_row >= 0 {
                ranges.push((sensor.pos.x - from_row)..(sensor.pos.x + from_row + 1));
            }
        }

        // sort_by performs better than sort_unstable_by on nearly-sorted slices
        // TODO: This is the slowest part of the loop, can this be optimized?
        ranges.sort_by(|a, b| a.start.cmp(&b.start));

        // Look for gaps, gaps should only be one element large
        let mut last_end_x = 0;
        for range in ranges.drain(..) {
            if range.start == last_end_x + 1 {
                return Some(calc_tuning_freq(last_end_x, y));
            }
            last_end_x = last_end_x.max(range.end);
        }

        // Also look for gaps at the end
        if last_end_x < expected_end {
            return Some(calc_tuning_freq(expected_end, y));
        }
    }

    None
}
