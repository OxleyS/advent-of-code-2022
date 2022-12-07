use crate::helpers::iterate_file_lines;

struct Directory {
    name: String,
    self_file_total: usize,
    parent: usize,
    child_dirs: Vec<usize>,
}

fn traverse_command_history() -> Vec<Directory> {
    let mut lines = iterate_file_lines("day7input.txt");

    // Populate the root directory immediately
    let mut directory_tree = vec![Directory {
        name: "/".to_string(),
        self_file_total: 0,
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
                            self_file_total: 0,
                            parent: cur_directory,
                            child_dirs: vec![],
                        });
                        directory_tree[cur_directory].child_dirs.push(child_idx);
                    }
                    file_size => {
                        directory_tree[cur_directory].self_file_total +=
                            file_size.parse::<usize>().expect("Invalid file size");
                    }
                }
            },
            _ => panic!("Unknown command"),
        }
    }

    directory_tree
}

fn sum_small_sizes(directory_tree: &[Directory]) -> usize {
    fn recurse(directory_tree: &[Directory], cur_idx: usize) -> (usize, usize) {
        let cur_dir = &directory_tree[cur_idx];
        let (child_size, child_sum) = cur_dir
            .child_dirs
            .iter()
            .map(|&child| recurse(directory_tree, child))
            .fold((0, 0), |accum, elem| (accum.0 + elem.0, accum.1 + elem.1));

        // Track the sum separately, because small dirs containing small dirs need to double-count
        let total_size = cur_dir.self_file_total + child_size;
        let total_small_sum = if total_size <= 100000 {
            child_sum + total_size
        } else {
            child_sum
        };

        (total_size, total_small_sum)
    }

    recurse(directory_tree, 0).1
}

pub fn solve_part1() {
    let directory_tree = traverse_command_history();
    println!("Sum is {}", sum_small_sizes(&directory_tree));
}
