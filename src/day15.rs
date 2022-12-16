use crate::helpers::iterate_file_lines;

#[derive(PartialEq, Eq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    pos: Coord,
    beacon_idx: usize,
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

        let beacon_idx = beacons.iter().position(|c| beacon_coord == *c).unwrap_or_else(|| {
            beacons.push(beacon_coord);
            beacons.len() - 1
        });

        sensors.push(Sensor { pos: sensor_coord, beacon_idx, manhattan });
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
    let (min_x, max_x) = sensors
        .iter()
        .filter_map(|sensor| {
            let from_row: i32 = sensor.manhattan - (y - sensor.pos.y);
            if from_row < 0 {
                return None;
            }
            Some((sensor.pos.x - from_row, sensor.pos.x + from_row))
        })
        .fold((i32::MAX, i32::MIN), |accum, elem| (accum.0.min(elem.0), accum.1.max(elem.1)));

    dbg!(min_x, max_x);

    (min_x..=max_x)
        .into_iter()
        .filter(|&x| {
            let cur = Coord { x, y };
            if beacons.contains(&cur) {
                return false;
            }

            sensors.iter().any(|sensor| manhattan_distance(&cur, &sensor.pos) <= sensor.manhattan)
        })
        .count()
}

#[derive(Debug, Clone)]
struct Box {
    x: std::ops::Range<i32>,
    y: std::ops::Range<i32>,
}

fn solve_part2(sensors: &[Sensor], full_range: std::ops::Range<i32>) -> Option<isize> {
    let mut boxes = vec![Box { x: full_range.clone(), y: full_range }];
    for sensor in sensors {
        let half_size = (((sensor.manhattan + 1) / 2) - 1).max(0);
        let contained_square = Box {
            x: (sensor.pos.x - (half_size - 1))..(sensor.pos.x + half_size),
            y: (sensor.pos.y - (half_size - 1))..(sensor.pos.y + half_size),
        };

        dbg!(half_size);

        let mut new_boxes = Vec::<Box>::new();
        for cur_box in boxes.iter() {
            // Disregard the non-intersecting cases
            if contained_square.y.end <= cur_box.y.start
                || contained_square.y.start >= cur_box.y.end
                || contained_square.x.end <= cur_box.x.start
                || contained_square.x.start >= cur_box.x.end
            {
                new_boxes.push(cur_box.clone());
                continue;
            }

            let intersect_y = contained_square.y.start.max(cur_box.y.start)
                ..contained_square.y.end.min(cur_box.y.end);
            let intersect_x = contained_square.x.start.max(cur_box.x.start)
                ..contained_square.x.end.min(cur_box.x.end);

            if intersect_y.start > cur_box.y.start {
                // Add top box
                new_boxes.push(Box { x: cur_box.x.clone(), y: cur_box.y.start..intersect_y.start });
            }

            if intersect_y.end < cur_box.y.end {
                // Add bottom box
                new_boxes.push(Box { x: cur_box.x.clone(), y: intersect_y.end..cur_box.y.end });
            }

            if intersect_x.start > cur_box.x.start {
                // Add left box
                new_boxes
                    .push(Box { x: cur_box.x.start..intersect_x.start, y: intersect_y.clone() });
            }

            if intersect_x.end < cur_box.x.end {
                // Add right box
                new_boxes.push(Box { x: intersect_x.end..cur_box.x.end, y: intersect_y.clone() });
            }
        }

        boxes = new_boxes;
    }

    println!(
        "Searching {} positions instead of {}",
        boxes
            .iter()
            .map(|b| ((b.x.end - b.x.start - 1) as usize * (b.y.end - b.y.start - 1) as usize))
            .sum::<usize>(),
        4000000usize * 4000000usize,
    );

    //dbg!(&boxes);

    for cur_box in boxes {
        for y in cur_box.y.clone() {
            for x in cur_box.x.clone() {
                let cur = Coord { x, y };
                if sensors
                    .iter()
                    .all(|sensor| manhattan_distance(&cur, &sensor.pos) > sensor.manhattan)
                {
                    let tuning_freq: isize = ((x as isize) * 4000000) + (y as isize);
                    return Some(tuning_freq);
                }
            }
        }
    }

    None
}
