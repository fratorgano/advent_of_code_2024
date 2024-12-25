pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    check_key_into_lock(parsed.0, parsed.1)
}

fn check_key_into_lock(keys: Vec<Vec<u8>>, locks: Vec<Vec<u8>>) -> usize {
    let mut tot = 0;
    for key in keys {
        for lock in &locks {
            let res = key.iter().zip(lock).map(|x| x.0 + x.1).all(|x| x < 6);
            if res {
                tot += 1;
            }
        }
    }
    tot
}

fn parse(strings: &[String]) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut keys = vec![];
    let mut locks = vec![];
    let mut is_key = None;
    let mut heights: Vec<u8> = vec![0, 0, 0, 0, 0];
    for line in strings {
        if line.is_empty() {
            if is_key.is_some() {
                if is_key.unwrap() {
                    keys.push(heights.iter().map(|x| x - 1).collect());
                } else {
                    locks.push(heights);
                }
            }
            heights = vec![0, 0, 0, 0, 0];
            is_key = None;
            continue;
        }
        if is_key.is_none() {
            if line == "#####" {
                is_key = Some(false)
            } else {
                is_key = Some(true)
            }
            continue;
        }
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                heights[i] += 1;
            }
        }
    }
    if is_key.is_some() {
        if is_key.unwrap() {
            keys.push(heights.iter().map(|x| x - 1).collect());
        } else {
            locks.push(heights);
        }
    }
    (keys, locks)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"
        .lines()
        .map(|s| String::from(s.trim()))
        .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(3, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(42, part2(&get_input()));
    }
}
