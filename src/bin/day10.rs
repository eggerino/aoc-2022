use aoc_2022::get_file_content;

enum Operation {
    Noop,
    Addx(i32),
}

impl Operation {
    fn parse(line: &str) -> Option<Self> {
        match line.split(" ").collect::<Vec<_>>()[..] {
            ["noop"] => Some(Self::Noop),
            ["addx", value] => match value.parse() {
                Ok(value) => Some(Self::Addx(value)),
                _ => None,
            },
            _ => None,
        }
    }
}

struct CpuState {
    register_x: i32,
    cycle: i32,
}

impl CpuState {
    fn signal_strength(&self) -> i32 {
        self.cycle * self.register_x
    }
}

struct Cpu {
    state: CpuState,
}

impl Cpu {
    fn new() -> Self {
        Self {
            state: CpuState {
                register_x: 1,
                cycle: 0,
            },
        }
    }

    fn execute<P, F>(&mut self, program: P, mut on_cycle: F)
    where
        P: Iterator<Item = Operation>,
        F: FnMut(&CpuState),
    {
        for operation in program {
            match operation {
                Operation::Noop => {
                    self.execute_cycle(&mut on_cycle);
                }
                Operation::Addx(value) => {
                    self.execute_cycle(&mut on_cycle);
                    self.execute_cycle(&mut on_cycle);
                    self.state.register_x += value;
                }
            }
        }
    }

    fn execute_cycle<F>(&mut self, on_cycle: &mut F)
    where
        F: FnMut(&CpuState),
    {
        self.state.cycle += 1;
        on_cycle(&self.state);
    }
}

struct Crt {
    current_position: i32,
}

impl Crt {
    fn new() -> Self {
        Self {
            current_position: 0,
        }
    }

    pub fn draw(&mut self, stride_position: i32) {
        let horizontal_beam_position = self.current_position % 40;
        let is_pixel_lit = (stride_position - horizontal_beam_position).abs() <= 1;

        if is_pixel_lit {
            print!("#");
        } else {
            print!(".");
        }

        self.current_position += 1;
        if self.current_position % 40 == 0 {
            println!("");
        }
    }
}

fn main() {
    let content = get_file_content();

    part1(&content);
    part2(&content);
}

fn part1(content: &str) {
    let program = content.lines().map(|x| Operation::parse(x).unwrap());

    let mut cpu = Cpu::new();
    let mut total_signal_strength = 0;

    cpu.execute(program, |state| {
        if (state.cycle + 20) % 40 == 0 && state.cycle <= 220 {
            total_signal_strength += state.signal_strength();
        }
    });

    println!("Part 1: {}", total_signal_strength);
}

fn part2(content: &str) {
    let program = content.lines().map(|x| Operation::parse(x).unwrap());

    let mut cpu = Cpu::new();
    let mut crt = Crt::new();

    println!("Part2:");
    cpu.execute(program, move |state| {
        crt.draw(state.register_x);
    });
}
