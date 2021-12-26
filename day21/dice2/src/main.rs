use std::fs;
use std::collections::HashMap;

const BOARD_SIZE:u8 = 10;
const SCORE_TARGET:u8 = 21;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct GameState {
    position: [u8; 2],
    score: [u8; 2],
    player: usize,
    round: usize,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            position: [0; 2],
            score: [0; 2],
            player: 0,
            round: 0,
        }
    }
}

struct Game {
    cache: HashMap<GameState, [usize; 2]>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            cache: HashMap::new(),
        }
    }


    pub fn play(&mut self, game: &GameState) -> [usize; 2] {
        let mut r = [0, 0];

        let mut tmp_game = game.clone();

        tmp_game.round += 1;
        if tmp_game.round == 4 {
            tmp_game.score[tmp_game.player] += game.position[tmp_game.player] + 1;
            if tmp_game.score[tmp_game.player] >= SCORE_TARGET {
                r[tmp_game.player] += 1;
                return r;
            } else {
                tmp_game.player = (tmp_game.player + 1) % 2;
                tmp_game.round = 1;
            }
        }


        for roll in 1..=3 {
            let mut new_game = tmp_game.clone();
            new_game.position[new_game.player] = (new_game.position[new_game.player] + roll) % BOARD_SIZE;
            if let Some(rr) = self.cache.get(&new_game) {
                r[0] += rr[0];
                r[1] += rr[1];
            } else {
                let rr = self.play(&new_game);
                self.cache.insert(new_game, rr);
                r[0] += rr[0];
                r[1] += rr[1];
            }
        }

        r

    }
}

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let mut game = Game::new();
//    let mut game_cache: HashMap<GameState, [usize; 2]>;
    let mut first_round = GameState::new();

    for (p, l) in data_string.lines().enumerate() {
        let (_, rhs) = l.split_once("er ").unwrap();
        let (_, pos) = rhs.split_once(" starting position: ").unwrap();
        first_round.position[p] = pos.parse::<u8>().unwrap() - 1;
    }

    println!("Initial state:");
    println!("Player 1: Position {}", first_round.position[0] + 1);
    println!("Player 2: Position {}", first_round.position[1] + 1);

    let answer = game.play(&mut first_round);
    println!("P1 wins {}; P2 wins {}", answer[0], answer[1]);
    let mut winner = 1;
    if answer[0] > answer[1] { winner = 0; }
    println!("Player {} wins with {}", winner+1, answer[winner]);

}
