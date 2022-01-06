use std::fs;

struct Instruction {
    push: bool,
    param1: isize,
    param2: isize,
}

impl Instruction {
    pub fn print(&self) {
        if self.push {
            println!("push {} {}", self.param1, self.param2);
        } else {
            println!("pop {} {}", self.param1, self.param2);
        }
    }
}

fn read_prog(filename: &str) -> Vec<Instruction> {
    let data_string = fs::read_to_string(filename).expect("Can't open file");

    let mut reader = data_string.lines();
    let mut program: Vec<Instruction> = Vec::new();

    while let Some(line) = reader.next() {
        if line.len() == 0 {
            continue;
        }

        let (opcode, _) = line.split_once(" ").unwrap();
        if opcode == "inp" {
            let mut push = true;
            let div_num = reader
                            .nth(3)
                            .unwrap()
                            .split_once(" ")
                            .unwrap()
                            .1
                            .split_once(" ")
                            .unwrap()
                            .1;

            if div_num.trim() == "26" {
                push = false;
            }

            let param1 = reader
                            .next()
                            .unwrap()
                            .split_once(" ")
                            .unwrap()
                            .1
                            .split_once(" ")
                            .unwrap()
                            .1
                            .parse::<isize>()
                            .unwrap();

            let param2 = reader
                            .nth(9)
                            .unwrap()
                            .split_once(" ")
                            .unwrap()
                            .1
                            .split_once(" ")
                            .unwrap()
                            .1
                            .parse::<isize>()
                            .unwrap();

            reader.nth(1);

            program.push(Instruction {
                            push: push,
                            param1: param1,
                            param2: param2,
            });

        } else {
            panic!("Input file out of sync");
        }
    }

    program

}

fn run_rec(program: &Vec<Instruction>, level: usize, num: isize, stack: &Vec<isize>, result: &mut Vec<isize>) {
    println!("Trying {}", num);

    if level == 14 && stack.len() == 0 {
        result.push(num);
        println!("{}", num);
        return;
    }

    if program[level].push {
        for w in (1..=9).rev() {
            let new_num = num * 10 + w;
            let mut new_stack = stack.clone();
            new_stack.push(w + program[level].param2);
            run_rec(program, level + 1, new_num, &new_stack, result);
        }
    } else {
        let x = stack[stack.len() - 1];
//        println!("Popped {}", x);
        let next_digit = x + program[level].param1;
//        println!("next_digit is {}", next_digit);
        if next_digit < 1 || next_digit > 9 {
            return;
        } else {
            let new_num = num * 10 + next_digit;
            let mut new_stack = stack.clone();
            new_stack.pop();
            run_rec(program, level + 1, new_num, &new_stack, result);
        }
    }

}

fn run(program: &Vec<Instruction>) -> Vec<isize> {
    let mut result: Vec<isize> = Vec::new();
    let stack: Vec<isize> = Vec::new();
    run_rec(program, 0, 0, &stack, &mut result);
    result
}

fn main() {
    let program = read_prog("input.txt");

    for instr in program.iter() {
        instr.print();
    }

    let answer = run(&program);

    println!("Largest number is {}", answer[0]);
    println!("Smallest number is {}", answer.iter().last().unwrap())

}
