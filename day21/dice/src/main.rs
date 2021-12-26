use std::fs;

const BOARD_SIZE:usize = 10;
struct Player {
    num: usize,
    position: usize,
    score: usize,
}

impl Player {
    pub fn new(n: usize) -> Self {
        Player {
            num: n,
            position: 1,
            score: 0,
        }
    }

    pub fn move_piece(&mut self, n: usize) -> usize {
        self.position = (self.position -1 + n) % BOARD_SIZE + 1;
        self.score += self.position;

        self.score
    }
}

const NUM_FACES:usize = 100;

struct Dice {
    face: usize,
    rolled: usize,
}

impl Dice {
    pub fn new() -> Self {
        Dice {
            face: 0,
            rolled :0,
        }
    }

    pub fn roll(&mut self) -> usize {
        let r = self.face;
        self.rolled += 1;

        self.face = (self.face + 1) % NUM_FACES;

        r + 1
    }
}

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let mut players: Vec<Player> = Vec::new();

    for l in data_string.lines() {
        let (_, rhs) = l.split_once("er ").unwrap();
        let (n, p) = rhs.split_once(" starting position: ").unwrap();
        players.push(Player {
                        num: n.parse::<usize>().unwrap(),
                        position: p.parse::<usize>().unwrap(),
                        score: 0,
        });
    }


    for p in players.iter() {
        println!("Player {} starting position: {}", p.num, p.position);
    }
    println!();

    let mut dice = Dice::new();
    let mut have_winner = false;
    let mut winning_player = 0;

    while !have_winner {
        for p in players.iter_mut() {
            let r1 = dice.roll();
            let r2 = dice.roll();
            let r3 = dice.roll();
            let score = p.move_piece(r1 + r2 + r3);

            println!(" - Player {} rolls {}+{}+{} and moves to space {} for a total score of {}", p.num, r1, r2, r3, p.position, score);

            if score >= 1000 {
                have_winner = true;
                winning_player = p.num;
                break;
            }
        }

    }

    let mut other_score = 0;
    for p in players.iter() {
        if p.num != winning_player {
            other_score = p.score;
            break;
        }
    }

    println!("Asnwer is {} * {} = {}", other_score, dice.rolled, other_score * dice.rolled);
}
