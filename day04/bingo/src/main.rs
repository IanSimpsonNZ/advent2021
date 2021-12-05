use std::fs;

const CARD_SIZE:usize = 5;

#[derive(Clone)]
struct Square {
    pub value: u32,
    pub marked: bool,
}

#[derive(Clone)]
struct Card {
    squares: Vec<Square>,
    col_sum: Vec<usize>,
    row_sum: Vec<usize>,
    pub done: bool,
}

impl Card {
    pub fn new(size: usize) -> Card {
        Card {
            squares: Vec::new(),
            col_sum: vec![0; size],
            row_sum: vec![0; size],
            done: false,
        }
    }

    pub fn print(&self) {
        for (i, v) in self.squares.iter().enumerate() {
            print!("{:3}", v.value);
            if v.marked {
                print!("*");
            } else {
                print!(" ");
            }

            if (i + 1) % CARD_SIZE == 0 {
                println!();
            }
        }
    }

    pub fn add_num(&mut self, val: u32) {
        self.squares.push(Square{value: val, marked: false});
    }

    pub fn check_bingo(&mut self, val: u32) -> bool {
        for (i, square) in self.squares.iter_mut().enumerate() {
            if square.value == val {
                square.marked = true;

                let col = i % CARD_SIZE;
                self.col_sum[col] += 1;
                if self.col_sum[col] == CARD_SIZE {
                    return true;
                }

                let row = i / CARD_SIZE;
                self.row_sum[row] += 1;
                if self.row_sum[row] == CARD_SIZE {
                    return true;
                }
            }
        }

        false
    }

    pub fn has_data(&self) -> bool {
        self.squares.len() > 0
    }

    pub fn sum_unmarked(&self) -> u32 {
        let mut result = 0;
        for s in self.squares.iter() {
            if !s.marked {
                result += s.value;
            }
        }

        result
    }
}

fn main() {
    // Get ready to read lines
    let data_string = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut data_lines = data_string.lines();

    // Assume first line is the list of drawn numbers
    let numbers: Vec<u32> = data_lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();


    // Set up the cards
    let mut cards: Vec<Card> = Vec::new();
    let mut tmp_card = Card::new(CARD_SIZE);

    for line in data_lines {
        // Blank line after a card => store card, otherwise skip the line
        if line.len() == 0 {
            if tmp_card.has_data() {
                cards.push(tmp_card);
                tmp_card = Card::new(CARD_SIZE);
            }
        } else {
            for v_string in line.split_ascii_whitespace() {
                tmp_card.add_num(v_string.parse::<u32>().unwrap());
            }
        }
    }

    // Check if we have a final card to store (i.e. no blank line after last card)
    if tmp_card.has_data() {
        cards.push(tmp_card);
    }

/*
    println!();
    println!("{:?}", numbers);
    println!();

    for (i, c) in cards.iter().enumerate() {
        println!("Card: {}", i);
        c.print();
        println!();
    }
*/

    let mut last_winner = Card::new(CARD_SIZE);
    let mut last_number = 0;
    let mut have_a_winner = false;

    for n in numbers.into_iter() {

        for (i, c) in cards.iter_mut().enumerate() {
            if !c.done && c.check_bingo(n) {
                if !have_a_winner {
                    println!();
                    println!("Card {} has first bingo!", i);
                    c.print();
                    println!();

                    let unmarked = c.sum_unmarked();
                    println!("Answer: {} * {} = {}", unmarked, n, unmarked * n);
                    }

                have_a_winner = true;
                last_winner = c.clone();
                last_number = n;

                c.done = true;
            }
        }
    }

    if have_a_winner {
        println!();
        println!("Card with last bingo is");
        last_winner.print();
        println!();

        let unmarked = last_winner.sum_unmarked();
        println!("Answer: {} * {} = {}", unmarked, last_number, unmarked * last_number);
    }
}
