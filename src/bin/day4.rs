use aoc_2022::get_file_content;

struct Section {
    start: i32,
    end: i32,
}

impl Section {
    fn new(s: &str) -> Self {
        let (start, end) = s.split_once("-").unwrap();
        Self {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }

    fn fully_contains(&self, other: &Section) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn fully_contains_bidirectional(&self, other: &Section) -> bool {
        self.fully_contains(other) || other.fully_contains(self)
    }

    fn contains(&self, value: i32) -> bool {
        self.start <= value && self.end >= value
    }

    fn overlaps_with(&self, other: &Section) -> bool {
        self.contains(other.start) || self.contains(other.end) || other.fully_contains(self)
    }
}

fn main() {
    let content = get_file_content();

    let result_part1 = content
        .lines()
        .map(split_line)
        .map(|(first, second)| (Section::new(first), Section::new(second)))
        .filter(|(first, second)| first.fully_contains_bidirectional(second))
        .count();

    println!("Part 1: {result_part1}");

    let result_part2 = content
        .lines()
        .map(split_line)
        .map(|(first, second)| (Section::new(first), Section::new(second)))
        .filter(|(first, second)| first.overlaps_with(second))
        .count();
    println!("Part 2: {result_part2}");
}

fn split_line(line: &str) -> (&str, &str) {
    line.split_once(",").unwrap()
}
