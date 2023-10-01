use aoc_2022::get_file_content;

fn main() {
    let content = get_file_content();

    let part1_result = get_marker_position(&content, 4).expect("No 4 series marker found");
    println!("Part 1: {}", part1_result);

    let part2_result = get_marker_position(&content, 14).expect("No 14 series marker found");
    println!("Part 2: {}", part2_result);
}

fn get_marker_position(content: &str, num_series: usize) -> Option<usize> {
    (0..(content.len() - num_series))
        .map(|i| is_unique(&content[i..(i + num_series)]))
        .enumerate()
        .filter(|(_, x)| *x)
        .map(|(i, _)| i + num_series)
        .next()
}

fn is_unique(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let length = chars.len();
    for idx_current in 0..(length - 1) {
        for idx_check in (idx_current + 1)..length {
            if chars[idx_current] == chars[idx_check] {
                return false;
            }
        }
    }
    return true;
}
