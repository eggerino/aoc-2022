use std::collections::HashSet;

use aoc_2022::get_file_content;

#[derive(Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn parse(line: &str) -> Option<Vec<Self>> {
        match line.split(" ").collect::<Vec<_>>()[..] {
            [direction, amount] => match amount.parse::<usize>() {
                Ok(amount) => match direction {
                    "L" => Some(vec![Direction::Left; amount]),
                    "U" => Some(vec![Direction::Up; amount]),
                    "R" => Some(vec![Direction::Right; amount]),
                    "D" => Some(vec![Direction::Down; amount]),
                    _ => None,
                },
                Err(_) => None,
            },
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Segment {
    x: i32,
    y: i32,
}

impl Segment {
    fn move_along(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
        }
    }

    fn follow(&mut self, other: &Segment) {
        let delta_x = other.x - self.x;
        let delta_y = other.y - self.y;

        if delta_x.abs().max(delta_y.abs()) <= 1 {
            return;
        }

        self.x += step(delta_x);
        self.y += step(delta_y);
    }
}

fn step(x: i32) -> i32 {
    match x {
        i32::MIN..=-1 => -1,
        0 => 0,
        1..=i32::MAX => 1,
    }
}

struct Rope {
    head: Segment,
    intermediate: Vec<Segment>,
    tail: Segment,
}

impl Rope {
    fn new(x: i32, y: i32, number_intermediate_knots: usize) -> Self {
        Self {
            head: Segment { x, y },
            intermediate: vec![Segment { x, y }; number_intermediate_knots],
            tail: Segment { x, y },
        }
    }

    fn move_head_along(&mut self, direction: &Direction) {
        self.head.move_along(direction);
        let mut previous = &self.head;
        for knot in self.intermediate.iter_mut() {
            knot.follow(previous);
            previous = knot;
        }
        self.tail.follow(previous);
    }
}

fn main() {
    let content = get_file_content();

    println!("Part 1: {}", solve(&content, 0));
    println!("Part 2: {}", solve(&content, 8));
}

fn solve(content: &str, number_intermediate_knots: usize) -> usize {
    let directions = content
        .lines()
        .map(|x| Direction::parse(x).unwrap())
        .flatten();

    let mut rope = Rope::new(0, 0, number_intermediate_knots);
    let mut tail_history = HashSet::new();

    for direction in directions {
        rope.move_head_along(&direction);
        tail_history.insert(rope.tail.clone());
    }

    tail_history.len()
}
