use std::fs;
use std::collections::HashMap;

/*
fn dive(prev_char: char, next_char: char, rules: &HashMap<String, char>, unique_chars: &mut HashMap<char,usize>, depth: usize, max_depth: usize) {
    if depth == max_depth {
        if let Some(v) = unique_chars.get_mut(&prev_char) {
            *v += 1;
        }
        if let Some(v) = unique_chars.get_mut(&next_char) {
            *v += 1;
        }
    } else {
        let index = format!("{}{}", next_char, prev_char);
        let new_char = rules.get(&index).unwrap().clone();
        dive(prev_char, new_char, rules, unique_chars, depth + 1, max_depth);
        dive(new_char, next_char, rules, unique_chars, depth + 1, max_depth);
    }
}
*/

fn print_result(result: &HashMap<String, HashMap<char, usize>>) {
    for (poly, line) in result {
        print!("{}", poly);
        for (ch, val) in line {
            print!("   {}:({:4})", ch, val);
        }
        println!();
    }
}

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let mut lines = data_string.lines();

    let mut polymer = lines.next().unwrap().to_string();
    polymer = "NN".to_string();

    let mut rules:HashMap<String, char> = HashMap::new();

    lines.next().unwrap();

    let mut unique_chars:HashMap<char, usize> = HashMap::new();

    for line in lines {
        let rule = line.split_once(" -> ").unwrap();
        let rule_char = rule.1.chars().next().unwrap();
        rules.insert(rule.0.to_string(), rule_char);
        unique_chars.insert(rule_char, 0);
    }


    let mut result: HashMap<String, HashMap<char, usize>> = HashMap::new();

    for rule in &rules {
        let mut this_result = unique_chars.clone();
        if let Some(v) = this_result.get_mut(&rule.1) {
            *v = 1;
        } else {
            panic!("Can't find {}", &rule.1);
        }
        result.insert(rule.0.to_string(), this_result);
    }

    println!("Step 1");
    print_result(&result);

    for step in 2..=3 {
        let mut next_result = result.clone();

        for (seq, _result_line) in &result {
            if let Some(next_result_line) = next_result.get_mut(seq) {
                let mut seq_chars = seq.chars();
                let c1 = seq_chars.next().unwrap();
                let c2 = seq_chars.next().unwrap();
                let new_char = rules.get(seq).unwrap();

                if let Some(vals) = result.get(&format!("{}{}", c1, new_char)) {
                    for (ch, val) in vals {
                        if let Some(new_val) = next_result_line.get_mut(ch) {
                            *new_val += val;
                        } else {
                            panic!("Can't find {}", ch);
                        }
                    }
                } else {
                    panic!("Can't find {}", &format!("{}{}", c1, new_char));
                }

                if let Some(vals) = result.get(&format!("{}{}", new_char, c2)) {
                    for (ch, val) in vals {
                        if let Some(new_val) = next_result_line.get_mut(ch) {
                            *new_val += val;
                        } else {
                            panic!("Can't find {}", ch);
                        }
                    }
                } else {
                    panic!("Can't find {}", &format!("{}{}", new_char, c2));
                }


            } else {
                panic!("Can't find {}", seq);
            }
        }

        result = next_result;

        println!();
        println!("Step {}", step);
        print_result(&result);
    }


    let mut poly_char = polymer.chars();
    let mut prev_char = poly_char.next().unwrap();
    if let Some(val) = unique_chars.get_mut(&prev_char) {
        *val += 1;
    } else {
        panic!("Can't find {}", &prev_char);
    }

    for next_char in poly_char {

        println!("Checking {}{}", prev_char, next_char);

        if let Some(val) = unique_chars.get_mut(&next_char) {
            *val += 1;
        } else {
            panic!("Can't find {}", &next_char);
        }

        if let Some(vals) = result.get(&format!("{}{}", prev_char, next_char)) {
            for (ch, val) in vals {
                if let Some(v) = unique_chars.get_mut(ch) {
                    *v += val;
                } else {
                    panic!("Can't find {}", ch);
                }
            }
        }

        println!("{:?}", unique_chars);
        prev_char = next_char;
    }


//    println!("{}", polymer);
//    println!();
//    println!("{:?}", rules);
/*
    for _step in 1..=10 {

        let mut poly_char = polymer.chars();

        let mut prev_char = poly_char.next().unwrap();

//        let mut new_poly = prev_char.to_string();
//        unique_chars.insert(prev_char, 0);

        for next_char in poly_char {
//            unique_chars.insert(next_char, 0);
            println!("Prev / Next: {} / {}", prev_char, next_char);
            dive(prev_char, next_char, &rules, &mut unique_chars, 0, 40);
//            let index = format!("{}{}", prev_char, next_char);
//            new_poly += &rules.get(&index).unwrap().to_string();
            //        println!("{}", new_poly);
//            new_poly += &next_char.to_string();
            prev_char = next_char;
        }

 //       polymer = new_poly;
    }
*/

// count occurences of each character
//    for (ch, count) in unique_chars.iter_mut() {
//        *count = polymer.matches(*ch).count();
//    }

    let top_char = unique_chars.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let bottom_char = unique_chars.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();


    println!("Most frequent char is {} ({})", top_char.0, top_char.1);
    println!("Least frequent char is {} ({})", bottom_char.0, bottom_char.1);

    println!("Answer is {}", top_char.1 - bottom_char.1);
}
