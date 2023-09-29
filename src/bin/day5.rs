use aoc_2022::get_file_content;

struct Instruction {
    amount: usize,
    source: usize,
    target: usize,
}

impl Instruction {
    fn new(s: &str) -> Self {
        let tokens = s.split(" ").collect::<Vec<_>>();

        Self {
            amount: tokens[1].parse().unwrap(),
            source: tokens[3].parse::<usize>().unwrap() - 1,
            target: tokens[5].parse::<usize>().unwrap() - 1,
        }
    }
}

struct Stacks {
    data: Vec<Vec<char>>,
}

impl Stacks {
    fn new(s: &str) -> Self {
        let mut lines = s.lines().collect::<Vec<_>>();

        let slots = lines
            .pop()
            .unwrap()
            .split(" ")
            .filter(|x| !x.is_empty())
            .count();

        let mut stacks = vec![Vec::new(); slots];

        for line in lines.into_iter().rev() {
            let chars = line.chars().collect::<Vec<_>>();
            for i in 0..slots {
                let item = chars[1 + 4 * i];
                if item != ' ' {
                    stacks[i].push(item);
                }
            }
        }

        Self { data: stacks }
    }

    fn move_crates_single(&mut self, inst: &Instruction) {
        for _ in 0..inst.amount {
            let item = self.data[inst.source].pop().unwrap();
            self.data[inst.target].push(item);
        }
    }

    fn move_crates_batch(&mut self, inst: &Instruction) {
        let source_amount = self.data[inst.source].len();

        for i in source_amount - inst.amount .. source_amount {
            let item = self.data[inst.source][i];
            self.data[inst.target].push(item);
        }
        self.data[inst.source].truncate(source_amount - inst.amount)
    }

    fn to_top_level(&self) -> String {
        self.data.iter().map(|x| x.last().unwrap()).collect()
    }
}

fn main() {
    let content = get_file_content();

    part1(&content);
    part2(&content);
}

fn part1(content: &str) {
    let (stacks, instructions) = content.split_once("\n\n").unwrap();

    let mut stacks = Stacks::new(stacks);

    for instruction in instructions.lines().map(|x| Instruction::new(x)) {
        stacks.move_crates_single(&instruction);
    }

    println!("Part 1: {}", stacks.to_top_level())
}

fn part2(content: &str) {
    let (stacks, instructions) = content.split_once("\n\n").unwrap();

    let mut stacks = Stacks::new(stacks);

    for instruction in instructions.lines().map(|x| Instruction::new(x)) {
        stacks.move_crates_batch(&instruction);
    }

    println!("Part 2: {}", stacks.to_top_level())
}
