use std::collections::{BTreeMap, HashMap};

use crate::helpers::iterate_file_lines;

struct GridNode {
    cost_estimate: usize,
    edge_indices: Vec<usize>,
}

#[derive(Clone)]
struct PathNode {
    grid_idx: usize,
    shortest_len: usize,
    cost_estimate: usize,
}

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

    let shortest_from_all_lowest = all_lowest
        .iter()
        .filter_map(|&start| find_shortest_path_length(&grid, start, end_idx))
        .min()
        .unwrap();
    println!("Shortest path length from any lowest point is {shortest_from_all_lowest}");
}

fn build_graph(elevations: &[u8], grid_width: usize, end_idx: usize) -> Vec<GridNode> {
    let end_y = end_idx / grid_width;
    let end_x = end_idx - (end_y * grid_width);

    // Smash the specific elevations into a graph with links between nodes based on their
    // relative elevations. Also include an estimated distance to the goal
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

// A (shoddy) implementation of A*. There's so many cross-references involved between path nodes and
// grid nodes that choosing the right data structures to avoid brute-force search is very difficult
fn find_shortest_path_length(grid: &[GridNode], start_idx: usize, end_idx: usize) -> Option<usize> {
    let initial_node = PathNode {
        grid_idx: start_idx,
        shortest_len: 0,
        cost_estimate: grid[start_idx].cost_estimate,
    };

    // A map where the key is the estimated distance to the goal, and the value is a list of path
    // node indices that currently have that estimated distance.
    // Normally you'd use a priority queue or min-heap, but the closest thing to that in the
    // standard lib (`BinaryHeap`) does not have APIs supporting updates
    let mut path_map = BTreeMap::from([(initial_node.cost_estimate, vec![0usize])]);

    // A simple array of every path node we've visited so far
    let mut path_nodes = vec![initial_node];

    // A mapping of grid node indices -> path node indices
    let mut reverse_map = HashMap::from([(start_idx, 0)]);

    // The min-key value in the path map will point us to the nodes with the smallest estimated
    // distance
    while let Some((cur_estimate, mut candidate_indices)) = path_map.pop_first() {
        // TODO: A bit inefficient. Arbitrarily take out a node to process and put the rest back
        let cur_path_idx = candidate_indices.pop().unwrap();
        if !candidate_indices.is_empty() {
            path_map.insert(cur_estimate, candidate_indices);
        }

        let cur_path = path_nodes[cur_path_idx].clone();

        // Reached our goal?
        if cur_path.grid_idx == end_idx {
            return Some(cur_path.shortest_len);
        }

        // For all neighbors
        let cur_grid = &grid[cur_path.grid_idx];
        for &edge_idx in cur_grid.edge_indices.iter() {
            // Exclude the current path node, such a path would never be shorter
            if edge_idx == cur_path.grid_idx {
                continue;
            }

            let edge_grid = &grid[edge_idx];

            // Apply the grid-level difference in cost estimate, plus the step it takes to travel
            // to the edge node. Order of operations is important here to prevent unsigned overflow
            let new_estimate =
                cur_path.shortest_len + edge_grid.cost_estimate + 1 - cur_grid.cost_estimate;

            // If the corresponding path node exists, get it, otherwise create one and add it to
            // the path map
            let edge_path_idx = *reverse_map.entry(edge_idx).or_insert_with(|| {
                path_nodes.push(PathNode {
                    grid_idx: edge_idx,
                    shortest_len: cur_path.shortest_len + 1,
                    cost_estimate: new_estimate,
                });
                path_map.entry(new_estimate).or_default().push(path_nodes.len() - 1);
                path_nodes.len() - 1
            });

            let edge_path = &mut path_nodes[edge_path_idx];

            // Is this path to the edge node shorter than any found so far?
            if new_estimate < edge_path.cost_estimate {
                // TODO: More jank. Remove the node from its previous spot in the path map, if any
                if let Some(indices) = path_map.get_mut(&edge_path.cost_estimate) {
                    let found_idx = indices.iter().position(|&idx| idx == edge_path_idx);
                    if let Some(found_idx) = found_idx {
                        indices.swap_remove(found_idx);
                        if indices.is_empty() {
                            path_map.remove(&edge_path.cost_estimate);
                        }
                    }
                }

                // Update our estimates and the path map accordingly
                edge_path.shortest_len = cur_path.shortest_len + 1;
                edge_path.cost_estimate = new_estimate;
                path_map.entry(edge_path.cost_estimate).or_default().push(edge_path_idx);
            }
        }
    }

    // No path exists
    None
}
