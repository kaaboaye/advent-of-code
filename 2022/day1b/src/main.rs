use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let result = read_to_string("./input.txt")
        .unwrap()
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .filter(|&item| item != "")
                .map(|item| item.parse::<u32>().expect("Not an item?!!11"))
                .sum::<u32>()
        })
        .sorted_unstable_by(|a, b| Ord::cmp(b, a))
        .take(3)
        .sum::<u32>();

    dbg!(result);
}
