use std::fs;
use std::collections::HashMap;

const NUM_STEPS:usize = 40;

#[derive(Clone, Debug)]
struct Node {
    pub letters: HashMap<char, usize>,
    pub left: String,
    pub right: String,
}

impl Node {
    pub fn new(idx: &str, ch: char, unique_chars: &HashMap<char, usize>) -> Node {
        let mut new_node = Node {
            letters: HashMap::new(),
            left: format!("{}{}", idx.chars().nth(0).unwrap(), ch),
            right: format!("{}{}", ch, idx.chars().nth(1).unwrap()),
        };

        for ch in unique_chars.keys() {
            new_node.letters.insert(*ch, 0);
        }

        new_node
    }
}

fn add_result(result: &mut HashMap<char, usize>, add: &HashMap<char, usize>) {
    for (ch, v) in add {
        *result.get_mut(ch).unwrap() += v;
    }
}

fn calc_result(result: &mut Vec<HashMap<String, Node>>, rules: &HashMap<String, char>, unique_chars: &HashMap<char, usize>, index: &str, level: usize) -> HashMap<char, usize> {

    let right_char = index.chars().last().unwrap();

    if level == NUM_STEPS {
        let mut this_result: HashMap<char, usize> = HashMap::new();
        for ch in unique_chars.keys() {
            this_result.insert(*ch, 0);
        }
        *this_result.get_mut(&right_char).unwrap() += 1;

        return this_result;
    }

    let ch = rules.get(index).unwrap();
    let mut new_node = Node::new(index, *ch, unique_chars);

    if let Some(node) = result[level + 1].get(&new_node.left) {
        add_result(&mut new_node.letters, &node.letters);
    } else {
        add_result(&mut new_node.letters, &calc_result(result, rules, unique_chars, &new_node.left, level + 1));
    }

    if let Some(node) = result[level + 1].get(&new_node.right) {
        add_result(&mut new_node.letters, &node.letters);
    } else {
        add_result(&mut new_node.letters, &calc_result(result, rules, unique_chars, &new_node.right, level + 1));
    }

    result[level].insert(index.to_string(), new_node.clone());

    new_node.letters
}

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let mut lines = data_string.lines();
    let polymer = lines.next().unwrap().to_string();
    let mut rules:HashMap<String, char> = HashMap::new();
    lines.next().unwrap();

    let mut unique_chars:HashMap<char, usize> = HashMap::new();

    for line in lines {
        let rule = line.split_once(" -> ").unwrap();
        let rule_char = rule.1.chars().next().unwrap();
        rules.insert(rule.0.to_string(), rule_char);
        unique_chars.insert(rule_char, 0);
    }

    let mut result: Vec<HashMap<String, Node>> = Vec::new();
    for _ in 0..=NUM_STEPS {
        result.push(HashMap::new());
    }

    let mut poly_char = polymer.chars();
    let mut prev_char = poly_char.next().unwrap();

    *unique_chars.get_mut(&prev_char).unwrap() += 1;

    for next_char in poly_char {
        let index = format!("{}{}", prev_char, next_char);
        if let Some(node) = result[0].get(&index) {
            add_result(&mut unique_chars, &node.letters)
        } else {
            let sub_result = calc_result(&mut result, &rules, &unique_chars, &index, 0);
            add_result(&mut unique_chars, &sub_result);
        }

        prev_char = next_char;
    }

    let top_char = unique_chars.values().max_by(|a, b| a.cmp(&b)).unwrap();
    let bottom_char = unique_chars.values().min_by(|a, b| a.cmp(&b)).unwrap();

    println!("Answer is {}", top_char - bottom_char);
}


