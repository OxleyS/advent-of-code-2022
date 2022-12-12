use std::collections::BinaryHeap;

use crate::helpers::iterate_file_lines;

struct GridNode {
    cost_estimate: usize,
    edge_indices: Vec<usize>,
}

struct AStarNode {}

#[derive(Clone)]
struct PathNode {
    grid_idx: usize,
    parent_idx: Option<usize>,
    shortest_distance: usize,
    cost_estimate: usize,
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare in _reverse_ order, we want the smallest cost_estimate to win
        other.cost_estimate.cmp(&self.cost_estimate)
    }
}
impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.cost_estimate == other.cost_estimate
    }
}
impl Eq for PathNode {}

pub fn solve() {
    const LOWEST_ELEVATION: u8 = 97; // 'a'
    const HIGHEST_ELEVATION: u8 = 122; // 'z'
    const START_MARKER: u8 = 83; // 'S'
    const END_MARKER: u8 = 69; // 'E'

    // Read the file into a 1D grid of bytes, noting the width for indexing later
    let mut grid_width: Option<usize> = None;
    let mut elevations: Vec<u8> = iterate_file_lines("day12input.txt")
        .map(|line| line.into_bytes())
        .inspect(|bytes| {
            assert!(
                *grid_width.get_or_insert(bytes.len()) == bytes.len(),
                "Grid was not uniform width"
            )
        })
        .fold(Vec::new(), |mut v, bytes| {
            v.extend_from_slice(&bytes);
            v
        });
    let grid_width = grid_width.expect("File was empty");

    let start_idx = elevations.iter().position(|&b| b == START_MARKER).expect("Missing start");
    let end_idx = elevations.iter().position(|&b| b == END_MARKER).expect("Missing end");
    elevations[start_idx] = LOWEST_ELEVATION;
    elevations[end_idx] = HIGHEST_ELEVATION;

    let all_lowest: Vec<usize> = elevations
        .iter()
        .enumerate()
        .filter_map(|(idx, b)| if *b == LOWEST_ELEVATION { Some(idx) } else { None })
        .collect();

    let grid: Vec<GridNode> = build_graph(&elevations, grid_width, end_idx);

    let shortest_length =
        find_shortest_path_length(&grid, start_idx, end_idx).expect("No possible path");
    println!("Shortest path length from start is {shortest_length}");
}

fn build_graph(elevations: &[u8], grid_width: usize, end_idx: usize) -> Vec<GridNode> {
    let end_y = end_idx / grid_width;
    let end_x = end_idx - (end_y * grid_width);

    elevations
        .iter()
        .enumerate()
        .map(|(idx, elevation)| {
            let y_idx = idx / grid_width;
            let x_idx = idx - (y_idx * grid_width);

            // Manhattan distance
            let cost_estimate = end_y.abs_diff(y_idx) + end_x.abs_diff(x_idx);

            let mut edge_indices = Vec::with_capacity(4);

            // Left
            if idx % grid_width != 0 && elevations[idx - 1] <= elevation + 1 {
                edge_indices.push(idx - 1);
            }

            // Right
            if idx % grid_width != (grid_width - 1) && elevations[idx + 1] <= elevation + 1 {
                edge_indices.push(idx + 1);
            }

            // Up
            if idx >= grid_width && elevations[idx - grid_width] <= elevation + 1 {
                edge_indices.push(idx - grid_width);
            }

            // Down
            if idx + grid_width < elevations.len() && elevations[idx + grid_width] <= elevation + 1
            {
                edge_indices.push(idx + grid_width);
            }

            GridNode { cost_estimate, edge_indices }
        })
        .collect()
}

fn find_shortest_path_length(grid: &[GridNode], start_idx: usize, end_idx: usize) -> Option<usize> {
    let initial_node = PathNode {
        grid_idx: start_idx,
        parent_idx: None,
        shortest_distance: 0,
        cost_estimate: grid[start_idx].cost_estimate,
    };

    let mut open_list = vec![initial_node];
    let mut close_list = Vec::new();

    while !open_list.is_empty() {
        let (cur_idx, node) =
            open_list.iter().enumerate().min_by_key(|&node| node.1.cost_estimate).unwrap();
        if node.grid_idx == end_idx {
            let mut path_len = 0;
            let mut traverse_idx = node.parent_idx;
            while let Some(parent_idx) = traverse_idx.take() {
                path_len += 1;
                traverse_idx = open_list
                    .iter()
                    .chain(close_list.iter())
                    .find(|node| node.grid_idx == parent_idx)
                    .map(|node| node.parent_idx)
                    .unwrap();
            }
            return Some(path_len);
        }

        let cur_node = node.clone();
        close_list.push(node.clone());
        open_list.remove(cur_idx);

        let cur_grid = &grid[cur_node.grid_idx];
        for &edge_idx in cur_grid.edge_indices.iter() {
            let edge_grid = &grid[edge_idx];
            let new_estimate =
                cur_node.cost_estimate + edge_grid.cost_estimate - cur_grid.cost_estimate + 1;

            if let Some(open_idx) = open_list.iter().position(|node| node.grid_idx == edge_idx) {
                let open_node = &mut open_list[open_idx];
                if new_estimate < open_node.cost_estimate {
                    open_node.cost_estimate = new_estimate;
                    open_node.parent_idx = Some(cur_node.grid_idx);
                }
            } else if let Some(close_idx) =
                close_list.iter().position(|node| node.grid_idx == edge_idx)
            {
                let close_node = &mut close_list[close_idx];
                if new_estimate < close_node.cost_estimate {
                    close_node.cost_estimate = new_estimate;
                    close_node.parent_idx = Some(cur_node.grid_idx);
                    open_list.push(close_node.clone());
                    close_list.remove(close_idx);
                }
            } else {
                open_list.push(PathNode {
                    cost_estimate: new_estimate,
                    grid_idx: edge_idx,
                    parent_idx: Some(cur_node.grid_idx),
                    shortest_distance: 0,
                })
            }
        }
    }

    // No path exists
    None
}
