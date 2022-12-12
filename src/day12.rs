use std::collections::{BTreeMap, BinaryHeap, HashMap};

use crate::helpers::iterate_file_lines;

struct GridNode {
    cost_estimate: usize,
    edge_indices: Vec<usize>,
}

struct AStarNode {}

#[derive(Clone)]
struct PathNode {
    grid_idx: usize,
    shortest_len: usize,
    cost_estimate: usize,
}

// TODO: Zap?
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
        shortest_len: 0,
        cost_estimate: grid[start_idx].cost_estimate,
    };

    let mut path_map = BTreeMap::from([(initial_node.cost_estimate, vec![0usize])]);
    let mut path_nodes = vec![initial_node];

    let mut reverse_map = HashMap::new();
    reverse_map.insert(start_idx, 0);

    while let Some((cur_estimate, mut candidate_indices)) = path_map.pop_first() {
        // TODO: A bit jank
        let cur_path_idx = candidate_indices.pop().unwrap();
        if !candidate_indices.is_empty() {
            path_map.insert(cur_estimate, candidate_indices);
        }

        let cur_path = path_nodes[cur_path_idx].clone();
        if cur_path.grid_idx == end_idx {
            return Some(cur_path.shortest_len);
        }

        let cur_grid = &grid[cur_path.grid_idx];
        for &edge_idx in cur_grid.edge_indices.iter() {
            if edge_idx == cur_path.grid_idx {
                continue;
            }

            let edge_grid = &grid[edge_idx];
            let new_estimate =
                cur_path.cost_estimate + edge_grid.cost_estimate - cur_grid.cost_estimate + 1;

            let edge_path_idx = *reverse_map.entry(edge_idx).or_insert_with(|| {
                path_nodes.push(PathNode {
                    grid_idx: edge_idx,
                    shortest_len: cur_path.shortest_len + 1,
                    cost_estimate: new_estimate,
                });
                path_map.entry(new_estimate).or_default().push(path_nodes.len() - 1);
                path_nodes.len() - 1
            });

            // TODO: More jank
            let edge_path = &mut path_nodes[edge_path_idx];
            if new_estimate < edge_path.cost_estimate {
                if let Some(indices) = path_map.get_mut(&edge_path.cost_estimate) {
                    let found_idx = indices.iter().position(|&idx| idx == edge_path_idx);
                    if let Some(found_idx) = found_idx {
                        indices.swap_remove(found_idx);
                        if indices.is_empty() {
                            path_map.remove(&edge_path.cost_estimate);
                        }
                    }
                }

                edge_path.shortest_len = cur_path.shortest_len + 1;
                edge_path.cost_estimate = new_estimate;
                path_map.entry(edge_path.cost_estimate).or_default().push(edge_path_idx);
            }
        }
    }

    // No path exists
    None
}
