use std::{fs, ops::RangeInclusive};

struct Elves {
    range1: RangeInclusive<u8>,
    range2: RangeInclusive<u8>,
}

impl Elves {
    fn contains(&self) -> bool {
        if self.range1.contains(self.range2.start()) && self.range1.contains(self.range2.end())
            || self.range2.contains(self.range1.start()) && self.range2.contains(self.range1.end())
        {
            return true;
        }
        false
    }
    fn overlaps(&self) -> bool {
        for range in self.range2.clone() {
            if self.range1.contains(&range) {
                return true;
            }
        }
        for range in self.range1.clone() {
            if self.range2.contains(&range) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let elves = read_input("test.txt");

    println!(
        "part1 is {}",
        elves.iter().filter(|elf| elf.contains()).count()
    );
    println!(
        "part2 is {}",
        elves.iter().filter(|elf| elf.overlaps()).count()
    );
}

fn read_input(path: &str) -> Vec<Elves> {
    let input = fs::read_to_string(path).unwrap();
    let mut elves = Vec::new();

    for line in input.lines() {
        let ranges = line.split_once(',').unwrap();
        let range1 = ranges.0.split_once('-').unwrap();
        let range2 = ranges.1.split_once('-').unwrap();
        elves.push(Elves {
            range1: RangeInclusive::new(
                range1.0.parse::<u8>().unwrap(),
                range1.1.parse::<u8>().unwrap(),
            ),
            range2: RangeInclusive::new(
                range2.0.parse::<u8>().unwrap(),
                range2.1.parse::<u8>().unwrap(),
            ),
        })
    }

    elves
}
