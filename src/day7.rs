use crate::helpers::iterate_file_lines;

struct Directory {
    name: String,
    local_file_total: usize,
    subtree_file_total: usize,
    parent: usize,
    child_dirs: Vec<usize>,
}

fn fill_subtree_sizes(directory_tree: &mut [Directory]) {
    fn recurse(directory_tree: &mut [Directory], cur_idx: usize, offset: usize) -> usize {
        // Children are always further in the slice than their parents. We can avoid double-borrow
        // by splitting up the slice just after the parent
        let offset_idx = cur_idx - offset;
        let child_offset = cur_idx + 1;
        let (left, right) = directory_tree[offset_idx..].split_at_mut(1);

        let cur_dir = &mut left[0];
        let child_sum: usize = cur_dir
            .child_dirs
            .iter()
            .map(|&child| recurse(right, child, child_offset))
            .sum();

        cur_dir.subtree_file_total = cur_dir.local_file_total + child_sum;
        cur_dir.subtree_file_total
    }

    recurse(directory_tree, 0, 0);
}

fn traverse_command_history() -> Vec<Directory> {
    let mut lines = iterate_file_lines("day7input.txt");

    // Populate the root directory immediately
    let mut directory_tree = vec![Directory {
        name: "/".to_string(),
        local_file_total: 0,
        subtree_file_total: 0,
        parent: 0,
        child_dirs: Vec::new(),
    }];
    let mut cur_directory = 0usize;

    let mut command = lines.next().expect("Unexpected EOF");
    while !command.is_empty() {
        assert!(command.starts_with("$ "), "Expected line to be a command");
        let (cmd_type, cmd_rest) = command[2..].split_at(2);

        match cmd_type {
            "cd" => {
                let dir_name = &cmd_rest[1..];
                match dir_name {
                    "/" => {
                        cur_directory = 0;
                    }
                    ".." => {
                        cur_directory = directory_tree[cur_directory].parent;
                    }
                    _ => {
                        cur_directory = *directory_tree[cur_directory]
                            .child_dirs
                            .iter()
                            .find(|&&idx| directory_tree[idx].name == dir_name)
                            .expect("Unknown subdirectory")
                    }
                }
                command = lines.next().expect("Unexpected EOF");
            }
            "ls" => loop {
                let cur_line = lines.next().unwrap_or_default();
                if cur_line.is_empty() || cur_line.starts_with("$ ") {
                    command = cur_line;
                    break;
                }

                let (file_tag, file_name) = cur_line.split_once(' ').expect("Malformed ls line");
                match file_tag {
                    "dir" => {
                        let child_idx = directory_tree.len();
                        directory_tree.push(Directory {
                            name: file_name.to_string(),
                            local_file_total: 0,
                            subtree_file_total: 0,
                            parent: cur_directory,
                            child_dirs: vec![],
                        });
                        directory_tree[cur_directory].child_dirs.push(child_idx);
                    }
                    file_size => {
                        directory_tree[cur_directory].local_file_total +=
                            file_size.parse::<usize>().expect("Invalid file size");
                    }
                }
            },
            _ => panic!("Unknown command"),
        }
    }

    // Now traverse the tree and calculate subtree total sizes
    fill_subtree_sizes(&mut directory_tree);

    directory_tree
}

fn sum_small_sizes(directory_tree: &[Directory]) -> usize {
    fn recurse(directory_tree: &[Directory], cur_idx: usize) -> usize {
        let cur_dir = &directory_tree[cur_idx];
        let child_sum = cur_dir
            .child_dirs
            .iter()
            .map(|&child| recurse(directory_tree, child))
            .sum();

        // Small dirs containing small dirs need to double-count
        if cur_dir.subtree_file_total <= 100000 {
            child_sum + cur_dir.subtree_file_total
        } else {
            child_sum
        }
    }

    recurse(directory_tree, 0)
}

fn find_deletion_candidate_size(directory_tree: &[Directory]) -> usize {
    const TOTAL_AVAILABLE_SPACE: usize = 70000000;
    const NEEDED_SPACE: usize = 30000000;
    let left_to_free =
        NEEDED_SPACE - (TOTAL_AVAILABLE_SPACE - directory_tree[0].subtree_file_total);

    fn recurse(directory_tree: &[Directory], cur_idx: usize, target: usize) -> Option<usize> {
        let cur_dir = &directory_tree[cur_idx];

        let local = if cur_dir.subtree_file_total > target {
            Some(cur_dir.subtree_file_total)
        } else {
            None
        };

        cur_dir
            .child_dirs
            .iter()
            .map(|&child| recurse(directory_tree, child, target))
            .chain([local])
            .fold(None, |accum: Option<usize>, size| {
                accum.into_iter().chain(size).min()
            })
    }

    recurse(directory_tree, 0, left_to_free).expect("No suitable directory")
}

pub fn solve() {
    let directory_tree = traverse_command_history();
    println!("Sum is {}", sum_small_sizes(&directory_tree));
    println!(
        "Smallest deletion is {}",
        find_deletion_candidate_size(&directory_tree)
    );
}
