use std::fs;

fn read_file(filename: &str) -> Vec<Cube> {
    let data_string = fs::read_to_string(filename).expect("Can't open file");

    let mut cube_list:Vec<Cube> = Vec::new();

    for line in data_string.lines() {
        let mut new_cube = Cube::new();

        let (on, coords) = line.split_once(" ").unwrap();
        new_cube.on = on == "on";

        let mut coord = coords.split(",");

        let (check, xs) = coord.next().unwrap().split_once("=").unwrap();
        if check != "x" { panic!("Expected x coords"); }
        let (min_str, max_str) = xs.split_once("..").unwrap();
        new_cube.min.x = min_str.parse::<isize>().unwrap();
        new_cube.max.x = max_str.parse::<isize>().unwrap();

        let (check, ys) = coord.next().unwrap().split_once("=").unwrap();
        if check != "y" { panic!("Expected y coords"); }
        let (min_str, max_str) = ys.split_once("..").unwrap();
        new_cube.min.y = min_str.parse::<isize>().unwrap();
        new_cube.max.y = max_str.parse::<isize>().unwrap();

        let (check, zs) = coord.next().unwrap().split_once("=").unwrap();
        if check != "z" { panic!("Expected z coords"); }
        let (min_str, max_str) = zs.split_once("..").unwrap();
        new_cube.min.z = min_str.parse::<isize>().unwrap();
        new_cube.max.z = max_str.parse::<isize>().unwrap();

        // Leave in the check_50 call for part 1
//        if new_cube.check_50() {
            cube_list.push(new_cube);
//        }
    }

    cube_list
}

#[derive(Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Clone)]
struct Cube {
    on: bool,
    min: Point,
    max: Point,
}

impl Point {
    pub fn new() -> Self {
        Point {
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            on: false,
            min: Point::new(),
            max: Point::new(),
        }
    }

    pub fn print(&self) {
        if self.on {
            print!("on ");
        } else {
            print!("off ");
        }

        print!("x={}..{},", self.min.x, self.max.x);
        print!("y={}..{},", self.min.y, self.max.y);
        println!("z={}..{}", self.min.z, self.max.z);
    }

    pub fn intersects(&self, other: &Cube) -> bool {
        self.max.x >= other.min.x && self.min.x <= other.max.x &&
        self.max.y >= other.min.y && self.min.y <= other.max.y &&
        self.max.z >= other.min.z && self.min.z <= other.max.z
    }

    pub fn split(&self, other: &Cube) -> Vec<Cube> {
        let mut result:Vec<Cube> = Vec::new();
        let mut original = self.clone();

        if self.intersects(other) {

            if original.min.x < other.min.x {
                result.push(
                    Cube {
                        on: original.on,
                        min: original.min.clone(),
                        max: Point {
                            x: other.min.x - 1,
                            y: original.max.y,
                            z: original.max.z,
                        },
                    }
                );

                original.min.x = other.min.x;
            }

            if original.max.x > other.max.x {
               result.push(
                    Cube {
                        on: original.on,
                        min: Point {
                            x: other.max.x + 1,
                            y: original.min.y,
                            z: original.min.z,
                        },
                        max: original.max.clone(),
                   }
                );

                original.max.x = other.max.x;
            }

            if original.min.y < other.min.y {
               result.push(
                    Cube {
                        on: original.on,
                        min: original.min.clone(),
                        max: Point {
                            x: original.max.x,
                            y: other.min.y - 1,
                            z: original.max.z,
                        },
                   }
                );

                original.min.y = other.min.y;
            }

            if original.max.y > other.max.y {
               result.push(
                    Cube {
                        on: original.on,
                        min: Point {
                            x: original.min.x,
                            y: other.max.y + 1,
                            z: original.min.z,
                        },
                        max: original.max.clone(),
                   }
                );

                original.max.y = other.max.y;
            }

            if original.min.z < other.min.z {
               result.push(
                    Cube {
                        on: original.on,
                        min: original.min.clone(),
                        max: Point {
                            x: original.max.x,
                            y: original.max.y,
                            z: other.min.z - 1,
                        },
                   }
                );

                original.min.z = other.min.z;
            }

            if original.max.z > other.max.z {
               result.push(
                    Cube {
                        on: original.on,
                        min: Point {
                            x: original.min.x,
                            y: original.min.y,
                            z: other.max.z + 1,
                        },
                        max: original.max.clone(),
                   }
                );

                original.max.z = other.max.z;
            }

        } else {
            result.push(original);
        }

        result
    }

/*
    pub fn check_50(&self) -> bool {
        self.min.x > -51 &&
        self.min.y > -51 &&
        self.min.z > -51 &&
        self.max.x < 51 &&
        self.max.y < 51 &&
        self.max.z < 51
    }
*/
}

fn count_on(cubes: &Vec<Cube>) -> usize {
    let mut result = 0;

    for cube in cubes {
        if cube.on {
            result += (cube.max.x - cube.min.x + 1) *
                      (cube.max.y - cube.min.y + 1) *
                      (cube.max.z - cube.min.z + 1);
        }
    }

    result as usize
}

fn print_list(cube_list: &Vec<Cube>) {
    for (n, cube) in cube_list.iter().enumerate() {
        print!("{}: ", n);
        cube.print();
    }
}

fn main() {
    let cube_list = read_file("input.txt");

    print_list(&cube_list);
    println!("\n\n");


    let mut result: Vec<Cube> = Vec::new();
    let mut sequence = cube_list.iter();
    result.push(sequence.next().unwrap().clone());

    for next_cube in sequence {
        let mut new_result:Vec<Cube> = Vec::new();

        // turn off the matching cubes in all other shapes irrepsective
        // of the cube being "on" or "off".  This stops the overlap if the
        // cube is "on" so we can just add it once at the end.
        for cube in result.iter() {
            new_result.append(&mut cube.split(next_cube));
        }

        if next_cube.on {
            new_result.push(next_cube.clone());
        }

        result = new_result;
    }
    println!("{} cubes are on", count_on(&result));

}
