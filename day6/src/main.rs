use std::{collections::VecDeque, fs};

const PART1: usize = 4;
const PART2: usize = 14;

fn main() {
    let input = get_input("input.txt");
    println!("{}", find_packet(&input));
}

fn find_packet(input: &str) -> usize {
    let mut packet = VecDeque::<char>::with_capacity(3);
    let mut counter = 0;

    for char in input.chars() {
        match packet.contains(&char) {
            true => {
                while packet.pop_front().unwrap() != char {}
                packet.push_back(char);
                counter += 1;
            }
            false => {
                packet.push_back(char);
                counter += 1;
            }
        }
        if packet.len() == PART2 {
            println!("{:?}", packet);
            break;
        }
    }

    counter
}

fn get_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}
