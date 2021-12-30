use std::fs;

const PATHS: [[([usize; 7], usize); 15]; 14] =
[
// 0 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 0
        ([0, 1, 0, 0, 0, 0, 0],    1), // 1
        ([0, 1, 2, 0, 0, 0, 0],    3), // 2
        ([0, 1, 2, 3, 0, 0, 0],    5), // 3
        ([0, 1, 2, 3, 4, 0, 0],    7), // 4
        ([0, 1, 2, 3, 4, 5, 0],    9), // 5
        ([0, 1, 2, 3, 4, 5, 6],    10),// 6
        ([0, 1, 7, 0, 0, 0, 0],    3), // 7
        ([0, 1, 2, 8, 0, 0, 0],    5), // 8
        ([0, 1, 2, 3, 9, 0, 0],    7), // 9
        ([0, 1, 2, 3, 4, 10, 0],   9), //10
        ([0, 1, 7, 11, 0, 0, 0],   4), //11
        ([0, 1, 2, 8, 12, 0, 0],   6), //12
        ([0, 1, 2, 3, 9, 13, 0],   8), //13
        ([0, 1, 2, 3, 4, 10, 14],  10),//14
    ],
// 1 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 0
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 1
        ([1, 2, 0, 0, 0, 0, 0],    2), // 2
        ([1, 2, 3, 0, 0, 0, 0],    4), // 3
        ([1, 2, 3, 4, 0, 0, 0],    6), // 4
        ([1, 2, 3, 4, 5, 0, 0],    8), // 5
        ([1, 2, 3, 4, 5, 6, 0],    9), // 6
        ([1, 7, 0, 0, 0, 0, 0],    2), // 7
        ([1, 2, 8, 0, 0, 0, 0],    4), // 8
        ([1, 2, 3, 9, 0, 0, 0],    6), // 9
        ([1, 2, 3, 4, 10, 0, 0],   8), //10
        ([1, 7, 11, 0, 0, 0, 0],   3), //11
        ([1, 2, 8, 12, 0, 0, 0],   5), //12
        ([1, 2, 3, 9, 13, 0, 0],   7), //13
        ([1, 2, 3, 4, 10, 14, 0],  9), //14
    ],
// 2 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 0
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 1
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 2
        ([2, 3, 0, 0, 0, 0, 0],    2), // 3
        ([2, 3, 4, 0, 0, 0, 0],    4), // 4
        ([2, 3, 4, 5, 0, 0, 0],    6), // 5
        ([2, 3, 4, 5, 6, 0, 0],    7), // 6
        ([2, 7, 0, 0, 0, 0, 0],    2), // 7
        ([2, 8, 0, 0, 0, 0, 0],    2), // 8
        ([2, 3, 9, 0, 0, 0, 0],    4), // 9
        ([2, 3, 4, 10, 0, 0, 0],   6), //10
        ([2, 7, 11, 0, 0, 0, 0],   3), //11
        ([2, 8, 12, 0, 0, 0, 0],   3), //12
        ([2, 3, 9, 13, 0, 0, 0],   5), //13
        ([2, 3, 4, 10, 14, 0, 0],  7), //14
    ],
// 3 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 0
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 1
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 2
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 3
        ([3, 4, 0, 0, 0, 0, 0],    2), // 4
        ([3, 4, 5, 0, 0, 0, 0],    4), // 5
        ([3, 4, 5, 6, 0, 0, 0],    5), // 6
        ([3, 2, 7, 0, 0, 0, 0],    4), // 7
        ([3, 8, 0, 0, 0, 0, 0],    2), // 8
        ([3, 9, 0, 0, 0, 0, 0],    2), // 9
        ([3, 4, 10, 0, 0, 0, 0],   4), //10
        ([3, 2, 7, 11, 0, 0, 0],   5), //11
        ([3, 8, 12, 0, 0, 0, 0],   3), //12
        ([3, 9, 13, 0, 0, 0, 0],   3), //13
        ([3, 4, 10, 14, 0, 0, 0],  5), //14
    ],
// 4 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 0
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 1
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 2
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 3
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 4
        ([4, 5, 0, 0, 0, 0, 0],    2), // 5
        ([4, 5, 6, 0, 0, 0, 0],    3), // 6
        ([4, 3, 2, 7, 0, 0, 0],    6), // 7
        ([4, 3, 8, 0, 0, 0, 0],    4), // 8
        ([4, 9, 0, 0, 0, 0, 0],    2), // 9
        ([4, 10, 0, 0, 0, 0, 0],   2), //10
        ([4, 3, 2, 7, 11, 0, 0],   7), //11
        ([4, 3, 8, 12, 0, 0, 0],   5), //12
        ([4, 9, 13, 0, 0, 0, 0],   3), //13
        ([4, 10, 14, 0, 0, 0, 0],  3), //14
    ],
// 5 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 0
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 1
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 2
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 3
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 4
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 5
        ([5, 6, 0, 0, 0, 0, 0],    1), // 6
        ([5, 4, 3, 2, 7, 0, 0],    8), // 7
        ([5, 4, 3, 8, 0, 0, 0],    6), // 8
        ([5, 4, 9, 0, 0, 0, 0],    4), // 9
        ([5, 10, 0, 0, 0, 0, 0],   2), //10
        ([5, 4, 3, 2, 7, 11, 0],   9), //11
        ([5, 4, 3, 8, 12, 0, 0],   7), //12
        ([5, 4, 9, 13, 0, 0, 0],   5), //13
        ([5, 10, 14, 0, 0, 0, 0],  3), //14
    ],
// 6 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 0
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 1
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 2
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 3
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 4
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 5
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 6
        ([6, 5, 4, 3, 2, 7, 0],    9), // 7
        ([6, 5, 4, 3, 8, 0, 0],    7), // 8
        ([6, 5, 4, 9, 0, 0, 0],    5), // 9
        ([6, 5, 10, 0, 0, 0, 0],   3), //10
        ([6, 5, 4, 3, 2, 7, 11],   10),//11
        ([6, 5, 4, 3, 8, 12, 0],   8), //12
        ([6, 5, 4, 9, 13, 0, 0],   6), //13
        ([6, 5, 10, 14, 0, 0, 0],  4), //14
    ],
// 7 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 0
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 1
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 2
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 3
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 4
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 5
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 6
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 7
        ([7, 2, 8, 0, 0, 0, 0],    4), // 8
        ([7, 2, 3, 9, 0, 0, 0],    6), // 9
        ([7, 2, 3, 4, 10, 0, 0],   8), //10
        ([7, 11, 0, 0, 0, 0, 0],   1), //11
        ([7, 2, 8, 12, 0, 0, 0],   5), //12
        ([7, 2, 3, 9, 13, 0, 0],   7), //13
        ([7, 2, 3, 4, 10, 14, 0],  9), //14
    ],
// 8 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 0
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 1
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 2
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 3
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 4
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 5
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 6
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 7
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 8
        ([8, 3, 9, 0, 0, 0, 0],    4), // 9
        ([8, 3, 4, 10, 0, 0, 0],   6), //10
        ([8, 2, 7, 11, 0, 0, 0],   5), //11
        ([8, 12, 0, 0, 0, 0, 0],   1), //12
        ([8, 3, 9, 13, 0, 0, 0],   5), //13
        ([8, 3, 4, 10, 14, 0, 0],  7), //14
    ],
// 9 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 0
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 1
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 2
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 3
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 4
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 5
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 6
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 7
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 8
        ([0, 0, 0, 0, 0, 0, 0],    0), //dummy 9
        ([9, 4, 10, 0, 0, 0, 0],   4), //10
        ([9, 3, 2, 7, 11, 0, 0],   7), //11
        ([9, 3, 8, 12, 0, 0, 0],   5), //12
        ([9, 13, 0, 0, 0, 0, 0],   1), //13
        ([9, 4, 10, 14, 0, 0, 0],  5), //14
    ],
// 10 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 0
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 1
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 2
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 3
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 4
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 5
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 6
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 7
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 8
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 9
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 10
        ([10, 4, 3, 2, 7, 11, 0],   9), //11
        ([10, 4, 3, 8, 12, 0, 0],   7), //12
        ([10, 4, 9, 13, 0, 0, 0],   5), //13
        ([10, 14, 0, 0, 0, 0, 0],   1), //14
    ],
// 11 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 0
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 1
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 2
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 3
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 4
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 5
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 6
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 7
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 8
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 9
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 10
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 11
        ([11, 7, 2, 8, 12, 0, 0],   6), //12
        ([11, 7, 2, 3, 9, 13, 0],   8), //13
        ([11, 7, 2, 3, 4, 10, 14],  10),//14
    ],
// 12 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 0
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 1
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 2
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 3
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 4
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 5
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 6
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 7
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 8
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 9
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 10
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 11
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 12
        ([12, 8, 3, 9, 13, 0, 0],   6), //13
        ([12, 8, 3, 4, 10, 14, 0],  8), //14
    ],
// 13 to
    [ //  path                 dist
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 0
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 1
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 2
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 3
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 4
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 5
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 6
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 7
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 8
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 9
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 10
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 11
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 12
        ([0, 0, 0, 0, 0, 0, 0],     0), //dummy 13
        ([13, 9, 4, 10, 14, 0, 0],  6), //14
    ],
];

#[derive(Clone)]
struct Amphi {
    variant: char,
    pos: usize,
    cost: usize,
    moves: usize,
}

impl Amphi {
    fn get_cost(variant: char) -> usize {
        match variant {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => panic!("Invalid amphipod type"),
        }
    }

    fn get_home(&self) -> (usize, usize) {
        match self.variant {
            'A' => (7, 11),
            'B' => (8, 12),
            'C' => (9, 13),
            'D' => (10, 14),
            _ => panic!("Invalid amphipod type"),
        }
    }

    fn is_home(&self, game: &Game) -> bool {
        let (top, bottom) = self.get_home();

        self.pos == bottom || (self.pos == top && game.map[bottom] == self.variant)
    }
}

#[derive(Clone)]
struct Game {
    map: [char; 15],
    amphis: Vec<Amphi>,
    min_cost: Option<usize>,
    cost_so_far: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            map: ['.'; 15],
            amphis: Vec::new(),
            min_cost: None,
            cost_so_far: 0,
        }
    }

    pub fn print(&self) {
        println!("{}{}.{}.{}.{}.{}{}", self.map[0], self.map[1], self.map[2],
                                       self.map[3], self.map[4], self.map[5],
                                       self.map[6]);
        println!("  {} {} {} {}", self.map[7], self.map[8], self.map[9], self.map[10]);
        println!("  {} {} {} {}", self.map[11], self.map[12], self.map[13], self.map[14]);
        println!("min cost      : {:?}", self.min_cost);
        println!("cost so far   : {}", self.cost_so_far);
    }

    // check if path is clear and return number of steps
    // modified so I don't trip over myself
    fn get_path(&self, f: usize, t: usize, my_pos: usize) -> Option<usize> {
        if f == t {return None;}

        let (from, to) =
            if f > t {
                (t, f)
            } else {
                (f, t)
            };

        let (path, dist) = &PATHS[from][to];

        for (idx, step) in path.iter().enumerate() {
            if *step == my_pos {
                continue;
            }

            if *step == 0 && idx != 0 {
                break;
            }
            if self.map[*step] != '.' {
                return None;
            }
        }

        Some(*dist)
    }

    pub fn play(&self, level: usize) -> Option<usize> {

//        println!("\nLevel: {}", level);
//        self.print();

        // If we've already spent more than the target, abandon
        if let Some(min_c) = self.min_cost {
            if self.cost_so_far >= min_c {
//                println!("Cost too high");
                return None;
            }
        }


        // Check to see if all done
        let mut all_home = true;
        for amphi in self.amphis.iter() {
            all_home = all_home && amphi.is_home(&self);
        }

        if all_home {
//            println!("All home");
            return Some(self.cost_so_far);
        }


        // try to make first move for each amphi on the map
        // move following thos one is handles by recursion
        let mut min_cost = self.min_cost;
//        let mut next_round = self.clone();

        'amphi_loop: for (idx, amphi) in self.amphis.iter().enumerate() {

            if level == 0 {
                println!("{} - {:?}", idx, min_cost);
            }

            // check if finished moving
            if amphi.moves == 2 {
//                println!("{} already moved twice", amphi.variant);
                continue 'amphi_loop;
            }

            // recreate starting point for each openning move
            // need to carry over the latest min cost
            let mut next_round = self.clone();
            next_round.min_cost = min_cost;
            let mut new_amphi = amphi.clone();

            // Can we move to home burrow .. or did we start there?
            let (top, bottom) = amphi.get_home();

            if amphi.pos == bottom ||
                (amphi.pos == top && self.map[bottom] == amphi.variant) {
                    //                println!("Already home!");
                    new_amphi.moves = 2;
                    // OK to store this in the working copy as we don't want to re-test every time
                    next_round.amphis[idx] = new_amphi;

                    if let Some(cost) = next_round.play(level + 1) {
//                        println!("Amphi home - {}", cost);
                        if let Some(min_c) = min_cost {
                            if cost < min_c {
                                min_cost = Some(cost);
                            }
                        } else {
                            min_cost = Some(cost);
                        }
                    }

                    continue 'amphi_loop;
                }

            if let Some(steps) = next_round.get_path(amphi.pos, bottom, amphi.pos) {
                //                println!("Can see home - bottom");
                next_round.cost_so_far += steps * amphi.cost;
                new_amphi.pos = bottom;
                new_amphi.moves = 2;
                next_round.amphis[idx] = new_amphi;
                next_round.map[amphi.pos] = '.';
                next_round.map[bottom] = amphi.variant;

                if let Some(cost) = next_round.play(level + 1) {
//                    println!("Get to bottom - {}", cost);
                    if let Some(min_c) = min_cost {
                        if cost < min_c {
                            min_cost = Some(cost);
                        }
                    } else {
                        min_cost = Some(cost);
                    }
                }

                continue 'amphi_loop;
            }

            if next_round.map[bottom] == amphi.variant {
                if let Some(steps) = next_round.get_path(amphi.pos, top, amphi.pos) {
                    //                    println!("Can see home - top");
                    next_round.cost_so_far += steps * amphi.cost;
                    new_amphi.pos = top;
                    new_amphi.moves = 2;
                    next_round.amphis[idx] = new_amphi;
                    next_round.map[amphi.pos] = '.';
                    next_round.map[top] = amphi.variant;

                    if let Some(cost) = next_round.play(level + 1) {
//                        println!("Get to top - {}", cost);
                        if let Some(min_c) = min_cost {
                            if cost < min_c {
                                min_cost = Some(cost);
                            }
                        } else {
                            min_cost = Some(cost);
                        }
                    }

                    continue 'amphi_loop;
                }
            }

            // If we alreday moved once, and now can't move home, we have to wait
            if amphi.moves == 1 {
//                println!("Can't see home - waiting");
                continue 'amphi_loop;
            }

            // The hard bit - test moving to each of the available top spaces
            for destination in 0..=6 {
                let mut new_next_round = next_round.clone();
                new_next_round.min_cost = min_cost;
                let mut second_new_amphi = amphi.clone();

                if let Some(steps) = new_next_round.get_path(amphi.pos, destination, amphi.pos) {
                    //                    println!("Got path {} to {}", amphi.pos, destination);
                    new_next_round.cost_so_far += steps * amphi.cost;
                    second_new_amphi.pos = destination;
                    second_new_amphi.moves = 1;
                    new_next_round.amphis[idx] = second_new_amphi.clone();
                    new_next_round.map[amphi.pos] = '.';
                    new_next_round.map[destination] = amphi.variant;

                    if let Some(cost) = new_next_round.play(level + 1) {
                        if let Some(min_c) = min_cost {
                        if cost < min_c {
                            min_cost = Some(cost);
                        }
                        } else {
                            min_cost = Some(cost);
                        }
                    }
                }
            }

        }

        min_cost
    }
}

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let mut line = data_string.lines();
    line.next().unwrap();
    line.next().unwrap();

    let mut game = Game::new();
    let mut amphi_pos = 7;

    for _ in 0..=1 {
        let mut c_iter = line.next().unwrap().chars();
        for i in [3, 1, 1, 1].iter() {

            let c = c_iter.nth(*i).unwrap();
            if c < 'A' || c > 'D' {
                panic!("Invalid amphipod type");
            }

            game.amphis.push(
                Amphi {
                    variant: c,
                    pos: amphi_pos,
                    cost: Amphi::get_cost(c),
                    moves: 0,
                }
            );

            game.map[amphi_pos] = c;
            amphi_pos += 1;
        }
    }

    game.print();

    println!("Minimum energy is {}", game.play(0).unwrap());
}
