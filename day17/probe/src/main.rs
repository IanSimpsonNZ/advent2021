use std::fs;
use std::collections::HashSet;

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let (_, rhs) = data_string.split_once("x=").unwrap();
    let (x_min_str, rhs) = rhs.split_once("..").unwrap();
    let (x_max_str, rhs) = rhs.split_once(", y=").unwrap();
    let (y_min_str, y_max_str) = rhs.split_once("..").unwrap();

    let x_min = x_min_str.trim().parse::<isize>().unwrap();
    let x_max = x_max_str.trim().parse::<isize>().unwrap();
    let y_min = y_min_str.trim().parse::<isize>().unwrap();
    let y_max = y_max_str.trim().parse::<isize>().unwrap();

    println!("Trench is x: {} to {}; y: {} to {}", x_min, x_max, y_min, y_max);

    let min_x_vel = (2.0 * x_min as f64).sqrt().floor() as isize;
    let max_drop_vel = (2.0 * x_max as f64).sqrt().floor() as isize;

    println!("Min x velocity is {}", min_x_vel);
    println!("Max x velocity to drop in {}", max_drop_vel);

    let max_y_vel = -y_min - 1;

    println!("Max y velocity is {}", max_y_vel);

    let t_to_level = max_y_vel * 2 + 1;

    println!("Time to hit tench is {}s", t_to_level + 1);

    println!("Position in tench is ({},{})", (max_drop_vel * (max_drop_vel + 1)) / 2, -(max_y_vel + 1));

    println!("Initial velocity is {},{}", max_drop_vel, max_y_vel);
    println!("Max height is {}", (max_y_vel * (max_y_vel + 1)) / 2);

// Part 2

    // we get all the single shots for free!
    let mut initial_vels: HashSet<(isize, isize)> = HashSet::new();

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            initial_vels.insert((x, y));
        }
    }

    println!("{} single shots", initial_vels.len());

    // Get a list of all the x velocities
    // The maximum time to reach the trench on the x axis is the time taken
    // by min_x_vel, which is min_x_vel.

    let mut possible_x: Vec<(isize, isize)> = Vec::new();

    for x in x_min..=x_max {
        for t in 2..=max_drop_vel {

            let top_eqn = x + (t - 1) * t / 2;
            if top_eqn % t == 0 {
                possible_x.push((top_eqn / t, t));
            }
        }
    }

    println!("{:?}", possible_x);

    // for each possible x, find a possible y velocity
    // for negative initial y vel we solve for y = vt - (t - 1)t / 2
    // for positive initial y vel we know how long the probe takes to fall
    // back to level 0 (2 * inital y vel + 1) then we're just searching for
    // a negative y velocity as above but for time t - t_level

    for (x, t) in possible_x {
        for y in y_min..=y_max {
            let top_eqn = y + (t - 1) * t / 2;
            if top_eqn % t == 0  && top_eqn <= 0 {
                    println!("Down - v_x = {}, v_y = {}", x, top_eqn / t);
                    initial_vels.insert((x, top_eqn / t));
            }

            for u_y in 1..=max_y_vel {
                let t_level = 2 * u_y + 1;
                if t_level <= t {
                    let end_time = t - t_level;
                    let d = -u_y * end_time - end_time * (end_time + 1) / 2;
                    if d >= y_min && d <= y_max {
                        println!("Up  - v_x = {}, v_y = {}", x, u_y);
                        initial_vels.insert((x, u_y));
                    }
                }


                if x == t {
                    let mut step = -u_y - 1;
                    let mut y_pos = step;
                    'check_drop: while y_pos >= y_min {
                        if y_pos <= y_max {
                            println!("Plop - v_x = {}, v_y = {}", x, u_y);
                            initial_vels.insert((x, u_y));
                            break 'check_drop;
                        }

                        step -= 1;
                        y_pos += step;
                    }
                }
            }
        }
    }

    println!("Num initial values is {}", initial_vels.len());
}
