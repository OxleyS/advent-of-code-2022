use crate::helpers::iterate_file_lines;

fn is_visible(tree_grid: &[Vec<u8>], x: usize, y: usize, width: usize, height: usize) -> bool {
    let tree = tree_grid[y][x];

    // Left
    if (0..x).all(|tx| tree_grid[y][tx] < tree) {
        return true;
    }

    // Right
    if ((x + 1)..width).all(|tx| tree_grid[y][tx] < tree) {
        return true;
    }

    // Up
    if (0..y).all(|ty| tree_grid[ty][x] < tree) {
        return true;
    }

    // Down
    if ((y + 1)..height).all(|ty| tree_grid[ty][x] < tree) {
        return true;
    }

    false
}

fn calc_scenic_score(
    tree_grid: &[Vec<u8>],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> usize {
    let tree = tree_grid[y][x];

    // It's all about how many trees you can see. So if you're right next to a taller tree, that
    // side has a score of 1, not 0. To compensate, we add 1, but that also means we have to stop
    // testing just before the array edges to avoid double-count
    let left = (1..x).rev().take_while(|&tx| tree_grid[y][tx] < tree).count() + 1;
    let right = ((x + 1)..(width - 1)).take_while(|&tx| tree_grid[y][tx] < tree).count() + 1;
    let up = (1..y).rev().take_while(|&ty| tree_grid[ty][x] < tree).count() + 1;
    let down = ((y + 1)..(height - 1)).take_while(|&ty| tree_grid[ty][x] < tree).count() + 1;

    left * right * up * down
}

pub fn solve() {
    let tree_grid: Vec<Vec<u8>> =
        iterate_file_lines("day8input.txt").map(|line| line.into_bytes()).collect();
    let (width, height) = (tree_grid[0].len(), tree_grid.len());

    let mut sum = 0usize;
    for y in 0..height {
        for x in 0..width {
            if is_visible(&tree_grid, x, y, width, height) {
                sum += 1;
            }
        }
    }

    // Any edge tree has one side with a score of zero, so edges' scenic score is always zero.
    // Excluding them dodges the edge cases (no pun intended)
    let mut scenic_score = 0usize;
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            scenic_score = scenic_score.max(calc_scenic_score(&tree_grid, x, y, width, height));
        }
    }

    println!("Visible trees: {sum}");
    println!("Max scenic score: {scenic_score}");
}
