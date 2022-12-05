use std::{collections::VecDeque, fs, vec};

#[derive(Debug)]
struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    let (mut stacks, commands) = get_input("input.txt");
    move_crates(&mut stacks, commands);
    for stack in stacks {
        println!("{:?}", stack);
    }
}

fn move_crates(stacks: &mut [VecDeque<char>], commands: Vec<Command>) {
    for command in commands {
        let mut intermediate_stack = VecDeque::new();
        for _i in 0..command.amount {
            intermediate_stack.push_front(stacks[command.from - 1].pop_back().unwrap());
        }
        stacks[command.to - 1].append(&mut intermediate_stack);
    }
}

fn get_input(path: &str) -> (Vec<VecDeque<char>>, Vec<Command>) {
    let input = fs::read_to_string(path).unwrap();
    let mut input = input.lines();
    let mut stacks = vec![VecDeque::<char>::new(); 9];

    for line in input.by_ref().take(8) {
        for (counter, i) in (1..line.len()).step_by(4).enumerate() {
            let content = line.chars().nth(i).unwrap();
            if content != ' ' {
                stacks[counter].push_front(content);
            }
        }
    }

    input.next();
    input.next();

    let commands: Vec<Command> = input
        .map(|line| {
            let split: Vec<&str> = line.split(' ').collect();
            Command {
                amount: split[1].parse::<usize>().unwrap(),
                from: split[3].parse::<usize>().unwrap(),
                to: split[5].parse::<usize>().unwrap(),
            }
        })
        .collect();

    (stacks, commands)
}
