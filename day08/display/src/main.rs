use std::fs;

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Could not open file");

    let data:Vec<(Vec<&str>, Vec<&str>)> = data_string
        .lines()
        .map(|s| s.split_once(" | ").unwrap())
        .map(|(input_str, output_str)| ( input_str.split(" ").collect(),
                                         output_str.split(" ").collect()))
        .collect();

/*  - Part 1

    let num = output
        .iter()
        .fold(0, |acc, v1| acc +
            v1
            .iter()
            .fold(0, |acc2, o_str| acc2 +
                match o_str.len() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0
                })
            );

*/

/*
Decode the segments

  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg


7 - 1                   gives a
9 has 6 chars and contains 4
9 - 4 - a               gives g
3 has five chars and contains 1
3 - 1 - a - g           gives d
9 - 1 - a - d - g       gives b
5 has five chars and contains a, b, d and g - fith char is f
1 - f                   gives c
e is the only one left

*/

    let mut total = 0;

    for (input, output) in data {
        let one = input.iter().find(|&&s| s.len() == 2).unwrap();
        let four = input.iter().find(|&&s| s.len() == 4).unwrap();
        let seven = input.iter().find(|&&s| s.len() == 3).unwrap();
        let eight = input.iter().find(|&&s| s.len() == 7).unwrap();

        let nine = input
            .iter()
            .filter(|&&s| s.len() == 6)
            .find(|&&s| four
                .chars()
                .fold(true, |acc, c| acc && s.contains(c)))
            .unwrap();

        let three = input
            .iter()
            .filter(|&&s| s.len() == 5)
            .find(|&&s| one
                .chars()
                .fold(true, |acc, c| acc && s.contains(c)))
            .unwrap();


        let mut a = seven.to_string();
        a.retain(|c| !one.contains(c));

        let mut g = nine.to_string();
        g.retain(|c| !(four.contains(c) || a.contains(c)));

        let mut d = three.to_string();
        d.retain(|c| !(one.contains(c) || a.contains(c) || g.contains(c)));

        let mut b = nine.to_string();
        b.retain(|c| !(one.contains(c) || a.contains(c) || d.contains(c) || g.contains(c)));

        let abdg = format!("{}{}{}{}", a, b, d, g);
        let five = input
            .iter()
            .filter(|&&s| s.len() == 5)
            .find(|&&s| abdg
                .chars()
                .fold(true, |acc, c| acc && s.contains(c)))
            .unwrap();

        let mut f = five.to_string();
        f.retain(|c| !abdg.contains(c));

        let mut c = one.to_string();
        c.retain(|c| !f.contains(c));

        let mut e = "abcdefg".to_owned();
        let abcdfg = format!("{}{}{}{}{}{}", a, b, c, d, f, g);
        e.retain(|c| !abcdfg.contains(c));

// Have all the segments and 1, 3, 4, 5, 7, 8, 9 - need to create 0, 2, 6
        let zero = format!("{}{}{}{}{}{}", a, b, c, e, f, g);
        let two = format!("{}{}{}{}{}", a, c, d, e, g);
        let six = format!("{}{}{}{}{}{}", a, b, d, e,  f, g);


        let mut this_num = 0;
        let values = [&zero, *one, &two, *three, *four, *five, &six, *seven, *eight, *nine];

        for digit in output {
            this_num = this_num * 10 + values
                                        .iter()
                                        .position(|val| val.len() == digit.len() &&
                                                        val.chars().fold(true, |acc, c| acc && digit.contains(c))
                                        )
                                        .unwrap();
        }

        total += this_num;

    }


    println!("{:?}", total);
}
