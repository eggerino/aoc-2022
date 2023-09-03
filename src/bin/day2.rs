use aoc_2022::get_file_content;

use Choice::*;
enum Choice {
    Rock,
    Paper,
    Scissors,
}

use RoundResult::*;
enum RoundResult {
    Win,
    Draw,
    Loss,
}

impl Choice {
    fn play_against(&self, other: &Self) -> RoundResult {
        match (self, other) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Loss,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Loss,
            (Scissors, Rock) => Loss,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }

    fn score(&self, result: &RoundResult) -> u32 {
        let choice_value = match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        let result_value = match result {
            Win => 6,
            Draw => 3,
            Loss => 0,
        };
        choice_value + result_value
    }

    fn from(character: char) -> Self {
        match character {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => panic!("Invalid choice character"),
        }
    }
}

impl RoundResult {
    fn from(character: char) -> Self {
        match character{
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("Invalid choice character"),
        }
    }

    fn required_choice(&self, other: &Choice) -> Choice {
        match (self, other) {
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Draw, Rock) => Rock,
            (Draw, Paper) => Paper,
            (Draw, Scissors) => Scissors,
            (Loss, Rock) => Scissors,
            (Loss, Paper) => Rock,
            (Loss, Scissors) => Paper,
        }
    }
}

fn main() {
    let content = get_file_content();

    let result_part1 = content.lines()
        .map(|l| {
                let chars = l.split(" ").collect::<Vec<_>>();
                let choice = Choice::from(chars[1].chars().nth(0).unwrap());
                let other = Choice::from(chars[0].chars().nth(0).unwrap());
                let result = choice.play_against(&other);
                choice.score(&result)
            }
        )
        .sum::<u32>();
    println!("Part 1: {}", result_part1);

    let result_part2 = content.lines()
        .map(|l| {
                let chars = l.split(" ").collect::<Vec<_>>();
                let result = RoundResult::from(chars[1].chars().nth(0).unwrap());
                let other = Choice::from(chars[0].chars().nth(0).unwrap());
                let choice = result.required_choice(&other);
                choice.score(&result)
            }
        )
        .sum::<u32>();
    println!("Part 2: {}", result_part2);
}
