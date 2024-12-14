pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    let limits = (101, 103);
    let simulated = simulate_robots(&parsed, limits, 100);
    calculate_safety_factor(&simulated, limits) as usize
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);
    let limits = (101, 103);
    find_easter_egg(&parsed, limits);
    6377 // the result I found by manually watching the output
}

fn simulate_robots(robots: &Vec<Robot>, limit: (isize, isize), time: isize) -> Vec<Robot> {
    let mut simulated_robots = vec![];
    for robot in robots {
        simulated_robots.push(robot.simulate(limit, time));
    }
    simulated_robots
}

fn find_easter_egg(robots: &Vec<Robot>, limit: (isize, isize)) {
    let mut time = 0;
    let mut scores = vec![];
    // try a lot of options
    loop {
        time += 1;
        let mut simulated_robots = vec![];
        for robot in robots {
            simulated_robots.push(robot.simulate(limit, time));
        }
        scores.push((time, calculate_safety_factor(&simulated_robots, limit)));
        if time == 101 * 103 {
            break;
        }
    }
    // sort them by safety score, maybe a tree image could have a low score since most will be grouped up
    scores.sort_by(|x, y| x.1.cmp(&y.1));
    // print the first 50 and check manually if there's a tree in them
    for (i, (time, score)) in scores.iter().enumerate() {
        let simulated = simulate_robots(robots, limit, *time);
        println!("time: {}, score: {}", time, score);
        println!("{}", string_robot_map(&simulated, limit));
        if i == 50 {
            break;
        }
        println!();
        println!();
    }
}

fn calculate_safety_factor(robots: &Vec<Robot>, limit: (isize, isize)) -> u128 {
    let mut tot_robots_quadrants = vec![0, 0, 0, 0];
    for robot in robots {
        if let Some(quadrant) = robot.determine_quadrant(limit) {
            tot_robots_quadrants[quadrant] += 1;
        }
    }

    tot_robots_quadrants.iter().product()
}

fn string_robot_map(robots: &Vec<Robot>, limit: (isize, isize)) -> String {
    let mut s = "".to_string();
    for y in 0..limit.1 {
        for x in 0..limit.0 {
            let filtered_robots = robots.iter().filter(|r| r.pos == (x, y)).count();
            if filtered_robots == 0 {
                s.push_str(".");
            } else {
                s.push_str(&format!("*"));
            }
        }
        s.push_str("\n");
    }
    s.push_str("\n");
    return s;
}

#[derive(Debug)]
struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}
impl Robot {
    fn simulate(&self, limit: (isize, isize), time: isize) -> Robot {
        let new_x = (self.pos.0 + self.vel.0 * time).rem_euclid(limit.0);
        let new_y = (self.pos.1 + self.vel.1 * time).rem_euclid(limit.1);
        Robot {
            pos: (new_x, new_y),
            vel: self.vel,
        }
    }

    fn determine_quadrant(&self, limit: (isize, isize)) -> Option<usize> {
        if self.pos.0 < limit.0 / 2 {
            // first half
            if self.pos.1 < limit.1 / 2 {
                // top half
                return Some(0);
            } else if self.pos.1 > limit.1 / 2 {
                // bottom half
                return Some(1);
            }
        } else if self.pos.0 > limit.0 / 2 {
            // second half
            if self.pos.1 < limit.1 / 2 {
                // top half
                return Some(2);
            } else if self.pos.1 > limit.1 / 2 {
                // bottom half
                return Some(3);
            }
        }
        None
    }
}

fn parse(strings: &[String]) -> Vec<Robot> {
    let mut robots = vec![];
    for line in strings {
        let mut pos_vel_splitter = line.split(' ');
        let pos_string = pos_vel_splitter.next().unwrap();
        let vel_string = pos_vel_splitter.next().unwrap();

        let (_, pos_x_y_string) = pos_string.split_at(2);
        let mut x_y_splitter = pos_x_y_string.split(',');
        let pos_x: isize = x_y_splitter.next().unwrap().parse().unwrap();
        let pos_y = x_y_splitter.next().unwrap().parse().unwrap();

        let (_, pos_x_y_string) = vel_string.split_at(2);
        let mut x_y_splitter = pos_x_y_string.split(',');
        let vel_x: isize = x_y_splitter.next().unwrap().parse().unwrap();
        let vel_y = x_y_splitter.next().unwrap().parse().unwrap();

        robots.push(Robot {
            pos: (pos_x, pos_y),
            vel: (vel_x, vel_y),
        })
    }
    robots
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        let parsed = parse(&get_input());
        let limits = (11, 7);
        let simulated = simulate_robots(&parsed, limits, 100);
        println!("{}", string_robot_map(&simulated, limits));
        assert_eq!(12, calculate_safety_factor(&simulated, limits));
    }
}
