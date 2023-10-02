use std::collections::HashMap;

use aoc_2022::get_file_content;

fn main() {
    let content = get_file_content();

    let dir_map = build_dir_map(&content);

    let result_part1 = dir_map.values().filter(|x| **x <= 100_000).sum::<i32>();
    println!("Part 1: {}", result_part1);

    let currently_free_space = 70_000_000 - dir_map.get("/").unwrap();
    let need_to_free = 30_000_000 - currently_free_space;
    let result_part2 = dir_map
        .values()
        .filter(|x| **x >= need_to_free)
        .min()
        .unwrap();
    println!("Part 2: {}", result_part2);
}

fn build_dir_map(content: &str) -> HashMap<String, i32> {
    let mut dir_map = HashMap::new();
    let mut dir_stack = Vec::new();

    for line in content.lines().into_iter() {
        match line.split(" ").collect::<Vec<_>>()[..] {
            ["$", "cd", dir] => match dir {
                "/" => dir_stack = vec![String::from("/")],
                ".." => {
                    dir_stack.pop();
                }
                _ => dir_stack.push(String::from(dir)),
            },
            [size, _] if size.parse::<i32>().is_ok() => {
                let size = size.parse::<i32>().unwrap();
                let mut path = String::new();
                for dir in dir_stack.iter() {
                    path.push_str(dir);
                    *dir_map.entry(path.clone()).or_insert(0) += size;
                }
            }
            _ => (),
        }
    }

    dir_map
}
