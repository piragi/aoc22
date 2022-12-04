use std::fs;

fn main() {
    println!("{}", get_input("input.txt"));
}

fn get_input(path: &str) -> u32 {
    let input = fs::read_to_string(path).unwrap();
    let mut calories = Vec::new();
    let mut single_elf_calories = 0;

    for line in input.lines() {
        if line.is_empty() {
            calories.push(single_elf_calories);
            single_elf_calories = 0;
            continue;
        }
        single_elf_calories += line.parse::<u32>().unwrap();
    }
    calories.push(single_elf_calories);

    calories.sort();
    calories[calories.len() - 3..].iter().sum()
}
