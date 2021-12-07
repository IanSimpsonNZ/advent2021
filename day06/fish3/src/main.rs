use std::fs;

const NUM_DAYS:usize = 256;

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Could not open file");

    let fishes: Vec<usize> = data_string
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();

    let mut fish_life = [0; 9];
    let mut pointer = 0;

    for fish in fishes {
        fish_life[fish] += 1;
    }

    for _day in 1..=NUM_DAYS {
        let spawners = fish_life[pointer];
        pointer = (pointer + 1) % 9;
        fish_life[(pointer + 6) % 9] += spawners;
    }

    let result:usize = fish_life.iter().sum();
    println!("{} fishes", result);
}
