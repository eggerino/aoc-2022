use std::collections::{HashSet, VecDeque};

use aoc_2022::get_file_content;

fn parse_map(content: &str) -> Vec<Vec<char>> {
    content.lines().map(|x| x.chars().collect()).collect()
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn from_map(label: char, map: &Vec<Vec<char>>) -> Option<Self> {
        for x in 0..map.len() {
            for y in 0..map[x].len() {
                if map[x][y] == label {
                    return Some(Self { x, y });
                }
            }
        }
        None
    }

    fn is_at(&self, label: char, map: &Vec<Vec<char>>) -> bool {
        map[self.x][self.y] == label
    }

    fn get_possible_descending_moves(&self, map: &Vec<Vec<char>>) -> Vec<Self> {
        self.get_neighbors(map)
            .into_iter()
            .filter(|x| x.can_ascend_to(self, map))
            .collect()
    }

    fn get_neighbors(&self, map: &Vec<Vec<char>>) -> Vec<Self> {
        let mut positions = Vec::new();

        if self.x > 0 {
            positions.push(Self {
                x: self.x - 1,
                ..*self
            });
        }

        if self.x < (map.len() - 1) {
            positions.push(Self {
                x: self.x + 1,
                ..*self
            });
        }

        if self.y > 0 {
            positions.push(Self {
                y: self.y - 1,
                ..*self
            });
        }

        if self.y < (map[self.x].len() - 1) {
            positions.push(Self {
                y: self.y + 1,
                ..*self
            });
        }

        positions
    }

    fn can_ascend_to(&self, target: &Self, map: &Vec<Vec<char>>) -> bool {
        (target.get_elevation_level(map) - self.get_elevation_level(map)) <= 1
    }

    fn get_elevation_level(&self, map: &Vec<Vec<char>>) -> i32 {
        match map[self.x][self.y] {
            'S' => 'a' as i32,
            'E' => 'z' as i32,
            x => x as i32,
        }
    }
}

struct State {
    number_moves: usize,
    position: Position,
}

fn main() {
    let content = get_file_content();

    println!("Part 1: {}", solve_reverse(&content, 'S'));
    println!("Part 2: {}", solve_reverse(&content, 'a'));
}

fn solve_reverse(content: &str, start_label: char) -> usize {
    let map = parse_map(content);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(State {
        number_moves: 0,
        position: Position::from_map('E', &map).expect("No end position in the map"),
    });

    loop {
        let current = queue.pop_front().expect("No solution found");

        if !visited.insert(current.position.clone()) {
            continue;
        }

        if current.position.is_at(start_label, &map) {
            return current.number_moves;
        }

        for next in current.position.get_possible_descending_moves(&map) {
            queue.push_back(State {
                number_moves: current.number_moves + 1,
                position: next,
            })
        }
    }
}
