use std::fs;

fn main() {
    println!("{}", get_input_part2("test.txt"));
}

fn get_input(path: &str) -> u32 {
    let input = fs::read_to_string(path).unwrap();
    input
        .lines()
        .map(|line| {
            let split = line.split_at(line.len() / 2);
            let mut found = '0';
            for char in split.0.chars() {
                if split.1.contains(char) {
                    found = char;
                }
            }
            priority(found)
        })
        .sum()
}

fn priority(item: char) -> u32 {
    if item as u32 > 90 {
        item as u32 - 96
    } else {
        item as u32 - 65 + 27
    }
}

fn get_input_part2(path: &str) -> u32 {
    let input = fs::read_to_string(path).unwrap();
    let input: Vec<&str> = input.lines().collect();

    let mut all_common = Vec::new();
    for i in (0..input.len()).step_by(3) {
        let common: Vec<char> = input[i]
            .chars()
            .filter(|char| input[i + 1].contains(*char) && input[i + 2].contains(*char))
            .collect();
        all_common.push(common[0]);
    }
    all_common.iter().map(|char| priority(*char)).sum()
}
