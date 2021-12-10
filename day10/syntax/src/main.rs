use std::fs;

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Could not open file");

    let mut score1 = 0;
    let mut auto_score: Vec<usize> = Vec::new();

    for line in data_string.lines() {
        let mut stack: Vec<char> = Vec::new();

        'char_loop: for c in line.chars() {

            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {
                    if let Some(prev_bracket) = stack.pop() {

                        match c {
                            ')' => {
                                if prev_bracket != '(' {
                                    score1 += 3;
                                    stack.clear();
                                    break 'char_loop;
                                }
                            },
                            ']' => {
                                if prev_bracket != '[' {
                                    score1 += 57;
                                    stack.clear();
                                    break 'char_loop;
                                }
                            },
                            '}' => {
                                if prev_bracket != '{' {
                                    score1 += 1197;
                                    stack.clear();
                                    break 'char_loop;
                                }
                            },
                            '>' => {
                                if prev_bracket != '<' {
                                    score1 += 25137;
                                    stack.clear();
                                    break 'char_loop;
                                }
                            },
                            _ => {
                                println!("Invalid character: '{}'", c);
                                stack.clear();
                                break 'char_loop;
                            }
                        }

                    } else {
                        println!("Stack is empty");
                        break 'char_loop;
                    }
                },

            }
        }

// check for incomplete line
        let mut score2 = 0;

        loop {
            if let Some(prev_bracket) = stack.pop() {
                score2 *= 5;
                score2 += match prev_bracket {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => {
                        println!("Invalid character");
                        break;
                    }
                };
            } else {
                if score2 > 0 {
                    auto_score.push(score2);
                }
                break;
            }
        }
    }

    auto_score.sort();

    println!("Score for part 1 = {}", score1);
    println!("Score for part 2 = {}", auto_score[auto_score.len() / 2]);
}
