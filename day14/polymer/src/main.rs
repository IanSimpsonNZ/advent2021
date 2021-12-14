use std::fs;
use std::collections::HashMap;

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let mut lines = data_string.lines();

    let mut polymer = lines.next().unwrap().to_string();
    let mut rules:HashMap<String, char> = HashMap::new();

    let _ = lines.next().unwrap();

    for line in lines {
        let rule = line.split_once(" -> ").unwrap();
        rules.insert(rule.0.to_string(), rule.1.chars().next().unwrap());
    }

//    println!("{}", polymer);
//    println!();
//    println!("{:?}", rules);
    let mut unique_chars:HashMap<char, usize> = HashMap::new();

    for _step in 1..=10 {

        let mut poly_char = polymer.chars();

        let mut prev_char = poly_char.next().unwrap();

        let mut new_poly = prev_char.to_string();
        unique_chars.insert(prev_char, 0);

        for next_char in poly_char {
            unique_chars.insert(next_char, 0);
            //        println!("Next char: {}", next_char);
            let index = format!("{}{}", prev_char, next_char);
            new_poly += &rules.get(&index).unwrap().to_string();
            //        println!("{}", new_poly);
            new_poly += &next_char.to_string();
            prev_char = next_char;
        }

        polymer = new_poly;
    }


// count occurences of each character
    for (ch, count) in unique_chars.iter_mut() {
        *count = polymer.matches(*ch).count();
    }

    let top_char = unique_chars.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let bottom_char = unique_chars.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();


    println!("Most frequent char is {} ({})", top_char.0, top_char.1);
    println!("Least frequent char is {} ({})", bottom_char.0, bottom_char.1);

    println!("Answer is {}", top_char.1 - bottom_char.1);
}
