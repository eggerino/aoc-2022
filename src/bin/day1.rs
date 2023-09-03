use aoc_2022::get_file_content;

fn main() {
    let content = get_file_content();

    let mut calories = content
        .split("\n\n")
        .map(|b| {
            b.lines()
                .map(|x| {
                    x.parse::<i32>()
                        .expect("Wrong format. A number is expected")
                })
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    calories.sort_by(|a, b| b.cmp(a)); // Sort in descending order

    let result_part1 = calories
        .get(0)
        .expect("Input does not contain a single block");
    println!("Part 1: {}", result_part1);

    let result_part2 = calories.into_iter().take(3).sum::<i32>();
    println!("Part 2: {}", result_part2);
}
