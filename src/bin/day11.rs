use aoc_2022::get_file_content;

enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn parse(line: &str) -> Option<Self> {
        let tokens = line.rsplit(" ").take(2).collect::<Vec<_>>();

        if let [arg, op] = tokens[..] {
            match (op, arg, arg.parse()) {
                ("+", _, Ok(arg)) => Some(Self::Add(arg)),
                ("*", _, Ok(arg)) => Some(Self::Multiply(arg)),
                ("*", "old", Err(_)) => Some(Self::Square),
                _ => None,
            }
        } else {
            None
        }
    }

    fn apply(&self, item: u64) -> u64 {
        match self {
            Operation::Add(x) => item + x,
            Operation::Multiply(x) => item * x,
            Operation::Square => item * item,
        }
    }
}

struct PassQueueItem {
    target: usize,
    item: u64,
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_divisor: u64,
    pass_to_on_success: usize,
    pass_to_on_failure: usize,
    number_inspects: usize,
}

impl Monkey {
    fn parse(block: &str) -> Option<Self> {
        let mut lines = block.lines();

        lines.next()?; // First line is irrelevant

        let items = lines
            .next()?
            .rsplit(": ")
            .next()?
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        let operation = Operation::parse(&lines.next()?)?;

        let test_divisor = lines.next()?.rsplit(" by ").next()?.parse().unwrap();

        let pass_to_on_success = lines.next()?.rsplit(" monkey ").next()?.parse().unwrap();
        let pass_to_on_failure = lines.next()?.rsplit(" monkey ").next()?.parse().unwrap();

        Some(Self {
            items,
            operation,
            test_divisor,
            pass_to_on_success,
            pass_to_on_failure,
            number_inspects: 0,
        })
    }

    fn make_turn<F>(&mut self, handle_stress: &F) -> Vec<PassQueueItem>
    where
        F: Fn(u64) -> u64,
    {
        let mut pass_queue = Vec::new();

        for item in self.items.iter() {
            let mut item = self.operation.apply(item.clone());
            item = handle_stress(item);

            let target = if item % self.test_divisor == 0 {
                self.pass_to_on_success
            } else {
                self.pass_to_on_failure
            };

            pass_queue.push(PassQueueItem { target, item });
        }
        
        self.number_inspects += self.items.len();
        self.items.clear();

        pass_queue
    }
}

fn main() {
    let content = get_file_content();

    println!("Part 1: {}", solve(&content, 20, |_| Box::new(|x| x / 3)));
    println!("Part 2: {}", solve(&content, 10000, handle_stress_part2));
}

fn solve<F>(content: &str, number_rounds: usize, handle_stress_factory: F) -> usize
where
    F: Fn(&[Monkey]) -> Box<dyn Fn(u64) -> u64>,
{
    let mut monkeys = content
        .split("\n\n")
        .map(|x| Monkey::parse(x).unwrap())
        .collect::<Vec<_>>();

    let handle_stress = handle_stress_factory(&monkeys);

    for _ in 0..number_rounds {
        for i_monkey in 0..monkeys.len() {
            let pass_queue = monkeys[i_monkey].make_turn(&handle_stress);

            for pass_queue_item in pass_queue {
                monkeys[pass_queue_item.target]
                    .items
                    .push(pass_queue_item.item);
            }
        }
    }

    monkeys.sort_by_key(|x| x.number_inspects);
    monkeys
        .into_iter()
        .map(|x| x.number_inspects)
        .rev()
        .take(2)
        .fold(1, |s, x| s * x)
}

fn handle_stress_part2(monkeys: &[Monkey]) -> Box<dyn Fn(u64) -> u64> {
    let common_divisor = monkeys.iter().fold(1, |s, x| s * x.test_divisor);
    Box::new(move |x| x % common_divisor)
}
