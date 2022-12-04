use std::fs;

fn main() {
    let points = get_input("test.txt");
    println!("{}", points);
}

fn get_input(path: &str) -> u32 {
    let input = fs::read_to_string(path).unwrap();

    input
        .lines()
        .map(|line| {
            let moves = line.split_once(' ').unwrap();
            calc_points(moves.0, moves.1)
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

fn calc_points(opponent: &str, you: &str) -> u32 {
    let outcome = match you {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("unexpected char"),
    };

    let selection = match opponent {
        "A" => match you {
            "X" => 3,
            "Y" => 1,
            "Z" => 2,
            _ => panic!("unexpected char"),
        },
        "B" => match you {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("unexpected char"),
        },
        "C" => match you {
            "X" => 2,
            "Y" => 3,
            "Z" => 1,
            _ => panic!("unexpected char"),
        },
        _ => panic!("unexpected char"),
    };

    outcome + selection
}
