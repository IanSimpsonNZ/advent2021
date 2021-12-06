use std::fs;
use std::collections::VecDeque;

const NUM_DAYS:usize = 256;

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Could not open file");

    let fishes: Vec<usize> = data_string
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();



    let mut fish_life: VecDeque<usize> = vec![0; 9].into_iter().collect();

    for fish in fishes {
        fish_life[fish] += 1;
    }

    for _day in 1..=NUM_DAYS {
        if let Some(spawners) = fish_life.pop_front() {
            fish_life[6] += spawners;
            fish_life.push_back(spawners);
        } else {
            println!("Horrible error - fish_life queue is empty");
        }
    }

    let result:usize = fish_life.iter().sum();
    println!("{} fishes", result);
}
