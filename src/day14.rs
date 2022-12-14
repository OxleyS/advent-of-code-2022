use std::mem::swap;

use crate::helpers::iterate_file_lines;

#[derive(Clone, Copy, PartialEq, Eq)]
enum TileType {
    Open,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

struct BoundingBox {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
}

pub fn solve() {
    let paths: Vec<Vec<Coord>> = iterate_file_lines("day14input.txt")
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let (x, y) = coord.split_once(',').expect("Malformed coordinate");
                    Coord {
                        x: x.parse::<usize>().expect("Bad X"),
                        y: y.parse::<usize>().expect("Bad Y"),
                    }
                })
                .collect()
        })
        .collect();

    // Calculate a bounding box that contains all the points
    let bbox = paths.iter().flat_map(|inner| inner.iter()).fold(
        BoundingBox { left: usize::MAX, right: 0, up: usize::MAX, down: 0 },
        |bbox, coord| BoundingBox {
            left: bbox.left.min(coord.x),
            right: bbox.right.max(coord.x),
            up: bbox.up.min(coord.y),
            down: bbox.down.max(coord.y),
        },
    );

    // Construct a grid that fits the bounding box. Note that we ignore the lower Y bound because it
    // must be zero to support the sand source
    // Each tile is a bool that says whether it's occupied by rock or sand.
    // Add one in each dimension because path ends are inclusive
    let grid_width = bbox.right - bbox.left + 1;
    let mut grid: Vec<Vec<TileType>> = vec![vec![TileType::Open; grid_width]; bbox.down + 1];
    for [start, end] in paths.iter().flat_map(|path| path.as_slice().array_windows()) {
        let mut offset_start_x = start.x - bbox.left;

        if start.x == end.x {
            // Vertical (range may be backwards)
            let mut start_y = start.y;
            let mut end_y = end.y;
            if start_y > end_y {
                swap(&mut start_y, &mut end_y);
            }

            #[allow(clippy::needless_range_loop)]
            for y in start_y..=end_y {
                grid[y][offset_start_x] = TileType::Rock;
            }
        } else {
            // Horizontal (range may be backwards)
            let mut offset_end_x = end.x - bbox.left;
            if offset_start_x > offset_end_x {
                swap(&mut offset_start_x, &mut offset_end_x);
            }

            for x in offset_start_x..=offset_end_x {
                grid[start.y][x] = TileType::Rock;
            }
        }
    }

    println!("Before");
    for line in grid.iter() {
        println!(
            "{}",
            line.iter()
                .map(|b| match b {
                    TileType::Open => '.',
                    TileType::Rock => '#',
                    TileType::Sand => 'o',
                })
                .collect::<String>()
        );
    }

    // TODO: If I wanted to be clever, I could reverse the X and Y directions so we're not jumping
    // between slices all the time
    let sand_x = 500 - bbox.left;
    let mut rest_units = 0;
    'outer: loop {
        let mut sand_pos = Coord { x: sand_x, y: 0 };
        'inner: loop {
            if sand_pos.y == bbox.down {
                break 'outer;
            }
            if grid[sand_pos.y + 1][sand_pos.x] == TileType::Open {
                sand_pos.y += 1;
                continue;
            }

            if sand_pos.x == 0 {
                break 'outer;
            }
            if grid[sand_pos.y + 1][sand_pos.x - 1] == TileType::Open {
                sand_pos.x -= 1;
                sand_pos.y += 1;
                continue;
            }

            if sand_pos.x == grid_width - 1 {
                break 'outer;
            }

            if grid[sand_pos.y + 1][sand_pos.x + 1] == TileType::Open {
                sand_pos.x += 1;
                sand_pos.y += 1;
                continue;
            }

            grid[sand_pos.y][sand_pos.x] = TileType::Sand;
            rest_units += 1;
            break 'inner;
        }
    }

    println!("After");
    for line in grid.iter() {
        println!(
            "{}",
            line.iter()
                .map(|b| match b {
                    TileType::Open => '.',
                    TileType::Rock => '#',
                    TileType::Sand => 'o',
                })
                .collect::<String>()
        );
    }

    println!("L {}, R {}, U {}, D {}", bbox.left, bbox.right, bbox.up, bbox.down);

    println!("Units of sand that came to rest: {rest_units}");
}
