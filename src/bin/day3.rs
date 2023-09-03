use aoc_2022::get_file_content;

fn main() {
    let content = get_file_content();

    let result_part1 = content
        .lines()
        .map(to_priorities)
        .map(split_at_half)
        .map(|mut x| {
            get_duplicate(&mut x).expect("At least one duplicate within one rucksack is required")
        })
        .sum::<i32>();
    println!("Part 1: {}", result_part1);

    let result_part2 = content
        .lines()
        .map(to_priorities)
        .collect::<Vec<_>>()
        .chunks_mut(3)
        .map(|x| get_duplicate(x).expect("At least one duplicate per 3 rucksacks is required"))
        .sum::<i32>();
    println!("Part 2: {}", result_part2);
}

fn split_at_half(items: Vec<i32>) -> [Vec<i32>; 2] {
    let split_index = items.len() / 2;
    [
        Vec::from(&items[0..split_index]),
        Vec::from(&items[split_index..]),
    ]
}

fn get_item_priority(item: char) -> i32 {
    (item as i32)
        + (if item.is_lowercase() {
            1 - 'a' as i32
        } else {
            27 - 'A' as i32
        })
}

fn to_priorities(items: &str) -> Vec<i32> {
    items.chars().map(get_item_priority).collect::<Vec<_>>()
}

fn get_duplicate(groups: &mut [Vec<i32>]) -> Option<i32> {
    for group in groups[1..].iter_mut() {
        group.sort();
    }

    for item in groups[0].iter() {
        if groups[1..]
            .iter()
            .all(|other| other.binary_search(item).is_ok())
        {
            return Some(*item);
        }
    }
    None
}
