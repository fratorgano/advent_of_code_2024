// looked up help
use std::collections::{BTreeMap, HashMap, HashSet};

pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    calculate_score(parsed.0, parsed.1, 2)
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);
    calculate_score(parsed.0, parsed.1, 25)
}

fn calculate_score(codes: Vec<Vec<Digit>>, nums: Vec<usize>, move_robots: usize) -> usize {
    let mut tot = 0;
    for (i, code) in codes.iter().enumerate() {
        let path_length = shortest_paths(&code, move_robots);
        tot += nums[i] * path_length
    }
    tot
}

fn dijkstra<R: Robot + Eq + Clone + Ord + std::hash::Hash>(
    current_robot: R,
    depth: usize,
    end: &R,
    cache: &mut HashMap<(usize, Direction, Direction), usize>,
) -> usize {
    if depth == 0 {
        return 0;
    }
    let mut visisted = HashSet::new();
    let mut todo = BTreeMap::<_, _>::from([(0, vec![(current_robot, Direction::A)])]);

    let target = (end.clone(), Direction::A);
    while !todo.is_empty() {
        let (current_score, todos) = todo.pop_first().unwrap();

        for current in todos {
            if !visisted.insert(current.clone()) {
                continue;
            }

            if current == target {
                return current_score;
            }

            for next in current.0.neighbours(end) {
                if !visisted.contains(&next) {
                    let cache_key = (depth - 1, current.1.clone(), next.1.clone());
                    let cost = if let Some(&cost) = cache.get(&cache_key) {
                        cost
                    } else {
                        let mut cost = dijkstra(current.1.clone(), depth - 1, &next.1, cache);
                        if !(next.1 == Direction::A) {
                            cost += 1;
                        }
                        cache.insert(cache_key, cost);
                        cost
                    };
                    todo.entry(current_score + cost).or_default().push(next);
                }
            }
        }
    }
    unreachable!("None should get here")
}

fn shortest_paths(code: &[Digit], move_robots: usize) -> usize {
    let mut current = Digit::A;
    let mut cache = HashMap::new();
    let mut total = 0;
    for digit in code {
        total += dijkstra(current, move_robots + 1, &digit, &mut cache) + 1;
        current = *digit;
    }
    total
}

trait Robot {
    fn neighbours(&self, end: &Self) -> Vec<(Self, Direction)>
    where
        Self: Sized;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}
impl Robot for Digit {
    fn neighbours(&self, end: &Digit) -> Vec<(Digit, Direction)> {
        if self != end {
            match self {
                Digit::Zero => vec![(Self::A, Direction::Right), (Self::Two, Direction::Up)],
                Digit::One => vec![(Self::Four, Direction::Up), (Self::Two, Direction::Right)],
                Digit::Two => vec![
                    (Self::One, Direction::Left),
                    (Self::Zero, Direction::Down),
                    (Self::Three, Direction::Right),
                    (Self::Five, Direction::Up),
                ],
                Digit::Three => vec![
                    (Self::Two, Direction::Left),
                    (Self::A, Direction::Down),
                    (Self::Six, Direction::Up),
                ],
                Digit::Four => vec![
                    (Self::One, Direction::Down),
                    (Self::Five, Direction::Right),
                    (Self::Seven, Direction::Up),
                ],
                Digit::Five => vec![
                    (Self::Four, Direction::Left),
                    (Self::Two, Direction::Down),
                    (Self::Six, Direction::Right),
                    (Self::Eight, Direction::Up),
                ],
                Digit::Six => vec![
                    (Self::Five, Direction::Left),
                    (Self::Three, Direction::Down),
                    (Self::Nine, Direction::Up),
                ],
                Digit::Seven => vec![
                    (Self::Four, Direction::Down),
                    (Self::Eight, Direction::Right),
                ],
                Digit::Eight => vec![
                    (Self::Seven, Direction::Left),
                    (Self::Five, Direction::Down),
                    (Self::Nine, Direction::Right),
                ],
                Digit::Nine => vec![(Self::Eight, Direction::Left), (Self::Six, Direction::Down)],
                Digit::A => vec![(Self::Zero, Direction::Left), (Self::Three, Direction::Up)],
            }
        } else {
            vec![(self.clone(), Direction::A)]
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    A,
}
impl Robot for Direction {
    fn neighbours(&self, end: &Direction) -> Vec<(Direction, Direction)> {
        if self != end {
            match self {
                Direction::Up => vec![
                    (Direction::Down, Direction::Down),
                    (Direction::A, Direction::Right),
                ],
                Direction::Down => vec![
                    (Direction::Up, Direction::Up),
                    (Direction::Left, Direction::Left),
                    (Direction::Right, Direction::Right),
                ],
                Direction::Left => vec![(Direction::Down, Direction::Right)],
                Direction::Right => vec![
                    (Direction::Down, Direction::Left),
                    (Direction::A, Direction::Up),
                ],
                Direction::A => vec![
                    (Direction::Right, Direction::Down),
                    (Direction::Up, Direction::Left),
                ],
            }
        } else {
            vec![(self.clone(), Direction::A)]
        }
    }
}

fn parse(strings: &[String]) -> (Vec<Vec<Digit>>, Vec<usize>) {
    let mut parsed = vec![];
    let mut nums = vec![];
    for line in strings {
        let mut elem = vec![];
        let (num, _) = line.split_at(line.len() - 1);
        nums.push(num.parse().unwrap());
        for c in line.chars() {
            elem.push(match c {
                '0' => Digit::Zero,
                '1' => Digit::One,
                '2' => Digit::Two,
                '3' => Digit::Three,
                '4' => Digit::Four,
                '5' => Digit::Five,
                '6' => Digit::Six,
                '7' => Digit::Seven,
                '8' => Digit::Eight,
                '9' => Digit::Nine,
                'A' => Digit::A,
                _ => unreachable!("Unecpected Input"),
            })
        }
        parsed.push(elem);
    }
    (parsed, nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "029A
980A
179A
456A
379A"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(126384, part1(&get_input()));
    }
}
