use aoc_2022::get_file_content;

fn main() {
    let content = get_file_content();
    let map = parse_height_map(&content);

    part_1(&map);
    part_2(&map);
}

fn parse_height_map(content: &String) -> Vec<Vec<u32>> {
    content
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn is_visible_at(map: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let x_vec = vec![x];
    let y_vec = vec![y];

    is_visible_blocked_by(map, x, y, &(0..x).collect(), &y_vec)
        || is_visible_blocked_by(map, x, y, &((x + 1)..map.len()).collect(), &y_vec)
        || is_visible_blocked_by(map, x, y, &x_vec, &(0..y).collect())
        || is_visible_blocked_by(map, x, y, &x_vec, &((y + 1)..map[0].len()).collect())
}

fn is_visible_blocked_by(
    map: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    x_indices: &Vec<usize>,
    y_indices: &Vec<usize>,
) -> bool {
    for i_x in x_indices {
        for i_y in y_indices {
            if map[*i_x][*i_y] >= map[x][y] {
                return false;
            }
        }
    }
    return true;
}

fn part_1(map: &Vec<Vec<u32>>) {
    let mut count = 0;
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if is_visible_at(map, x, y) {
                count += 1;
            }
        }
    }
    println!("Part 1: {}", count);
}

fn scenic_score_at(map: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let x_vec = vec![x];
    let y_vec = vec![y];

    view_distance_at(map, x, y, &(0..x).rev().collect(), &y_vec)
        * view_distance_at(map, x, y, &((x + 1)..map.len()).collect(), &y_vec)
        * view_distance_at(map, x, y, &x_vec, &(0..y).rev().collect())
        * view_distance_at(map, x, y, &x_vec, &((y + 1)..map[0].len()).collect())
}

fn view_distance_at(
    map: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    x_indices: &Vec<usize>,
    y_indices: &Vec<usize>,
) -> u32 {
    let mut distance = 0;
    for i_x in x_indices {
        for i_y in y_indices {
            distance += 1;
            if map[*i_x][*i_y] >= map[x][y] {
                return distance;
            }
        }
    }
    distance
}

fn part_2(map: &Vec<Vec<u32>>) {
    let mut scores = Vec::new();

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            scores.push(scenic_score_at(map, x, y));
        }
    }
    println!("Part 2: {}", scores.iter().max().unwrap());
}
