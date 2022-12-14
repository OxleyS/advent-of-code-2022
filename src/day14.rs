use std::mem::swap;

use crate::helpers::iterate_file_lines;

#[derive(Clone, Copy, PartialEq, Eq)]
enum TileType {
    Open,
    Rock,
    Sand,
}

#[derive(Debug, Clone, Copy)]
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

const SAND_START: Coord = Coord { x: 500, y: 0 };

pub fn solve() {
    let paths = parse_paths();
    let bbox = calc_bounding_box(&paths);

    // Part 1
    {
        let mut grid = construct_grid(&paths, &bbox);
        let rest_before_abyss = simulate_sand(&mut grid, &bbox);
        println!("Units of sand that came to rest before the abyss: {rest_before_abyss}");
    }

    // Part 2
    {
        // Expand the bounding box to open up more blank space on the sides and bottom.
        // Now that we have a floor for sand to pile up on, we'll need to account for the max width
        // a sand pile could take up. The largest pile of sand possible is centered at the sand
        // source and goes all the way to the floor. In this case, a triangle is formed, and the
        // width is two times the height. Also add one extra so the sand simulation doesn't see
        // abyss before attempting (in vain) to go diagonally
        const EXTRA_HEIGHT: usize = 2;
        let worst_case_sand_width = bbox.down + EXTRA_HEIGHT + 1;
        let min_x = SAND_START.x.saturating_sub(worst_case_sand_width);
        let max_x = SAND_START.x.saturating_add(worst_case_sand_width);

        // Take care that this recalculation doesn't take away space where rocks need to go
        let mut bbox = bbox;
        bbox.left = bbox.left.min(min_x);
        bbox.right = bbox.right.max(max_x);
        bbox.down += EXTRA_HEIGHT;

        // Add a path for the floor
        let mut paths = paths;
        paths.push(vec![
            Coord { x: bbox.left, y: bbox.down },
            Coord { x: bbox.right, y: bbox.down },
        ]);

        let mut grid = construct_grid(&paths, &bbox);
        let rest_before_abyss = simulate_sand(&mut grid, &bbox);
        println!("Units of sand that came to rest before source block: {rest_before_abyss}");
    }
}

fn parse_paths() -> Vec<Vec<Coord>> {
    iterate_file_lines("day14input.txt")
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
        .collect()
}

// Calculate a bounding box that contains all the points
fn calc_bounding_box(paths: &[Vec<Coord>]) -> BoundingBox {
    paths.iter().flat_map(|inner| inner.iter()).fold(
        BoundingBox { left: usize::MAX, right: 0, up: usize::MAX, down: 0 },
        |bbox, coord| BoundingBox {
            left: bbox.left.min(coord.x),
            right: bbox.right.max(coord.x),
            up: bbox.up.min(coord.y),
            down: bbox.down.max(coord.y),
        },
    )
}

// Construct a grid that fits the bounding box. Note that we ignore the lower Y bound because it
// must be zero to support the sand source.
// Add one in each dimension because path ends are inclusive
fn construct_grid(paths: &[Vec<Coord>], bbox: &BoundingBox) -> Vec<Vec<TileType>> {
    let grid_width = bbox.right - bbox.left + 1;
    let mut grid: Vec<Vec<TileType>> = vec![vec![TileType::Open; grid_width]; bbox.down + 1];

    for [mut start, mut end] in
        paths.iter().flat_map(|path| path.as_slice().array_windows().copied())
    {
        start.x -= bbox.left;
        end.x -= bbox.left;

        if start.x == end.x {
            // Vertical (range may be backwards)
            if start.y > end.y {
                swap(&mut start.y, &mut end.y);
            }

            #[allow(clippy::needless_range_loop)]
            for y in start.y..=end.y {
                grid[y][start.x] = TileType::Rock;
            }
        } else {
            // Horizontal (range may be backwards)
            if start.x > end.x {
                swap(&mut start.x, &mut end.x);
            }

            for x in start.x..=end.x {
                grid[start.y][x] = TileType::Rock;
            }
        }
    }

    grid
}

fn simulate_sand(grid: &mut [Vec<TileType>], bbox: &BoundingBox) -> usize {
    // TODO: If I wanted to be clever, I could reverse the X and Y directions so we're not jumping
    // between slices all the time
    let grid_width = grid[0].len();
    let sand_start = Coord { x: SAND_START.x - bbox.left, y: SAND_START.y };
    let mut rest_units = 0;

    // Until the sand source is blocked
    'emit_loop: while grid[sand_start.y][sand_start.x] != TileType::Sand {
        let mut sand_pos = sand_start;
        'fall_loop: loop {
            // Reached the abyss?
            if sand_pos.y == bbox.down {
                break 'emit_loop;
            }

            // Straight down
            if grid[sand_pos.y + 1][sand_pos.x] == TileType::Open {
                sand_pos.y += 1;
                continue;
            }

            // Reached the left side abyss?
            // Don't test until we attempt to move down left
            if sand_pos.x == 0 {
                break 'emit_loop;
            }

            // Down left
            if grid[sand_pos.y + 1][sand_pos.x - 1] == TileType::Open {
                sand_pos.x -= 1;
                sand_pos.y += 1;
                continue;
            }

            // Reached the right side abyss?
            // Don't test until we attempt to move down right
            if sand_pos.x == grid_width - 1 {
                break 'emit_loop;
            }

            // Down right
            if grid[sand_pos.y + 1][sand_pos.x + 1] == TileType::Open {
                sand_pos.x += 1;
                sand_pos.y += 1;
                continue;
            }

            // Can't move anywhere, rest the sand here
            grid[sand_pos.y][sand_pos.x] = TileType::Sand;
            rest_units += 1;
            break 'fall_loop;
        }
    }

    rest_units
}

// For debugging
fn print_grid(grid: &[Vec<TileType>]) {
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
}
