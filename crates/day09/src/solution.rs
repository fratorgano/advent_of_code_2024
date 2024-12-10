use std::fmt::{self};

pub fn part1(input: &[String]) -> usize {
    let disk = parse(input);
    let compacted_disk = disk.compact_blocks();
    compacted_disk.calculate_checksum()
}

pub fn part2(input: &[String]) -> usize {
    let mut disk = parse2(input);
    // println!("{}", disk);
    disk.compact_files();
    disk.calculate_checksum()
}

#[derive(Debug)]
struct Disk2 {
    blocks: Vec<(Option<usize>, usize)>,
    max_file_id: usize,
}

impl Disk2 {
    fn compact_files(&mut self) {
        let mut last_file_id = self.max_file_id + 1;

        loop {
            let last_file = self.get_last_file(last_file_id);
            last_file_id = last_file.0;
            let last_file_size = last_file.1;
            let last_file_index = last_file.2;
            let mut new_index = None;
            let mut space_size = 0;
            for (i, (file_id_opt, size)) in (&self).blocks.iter().enumerate() {
                if file_id_opt.is_none() && last_file_size <= *size {
                    // println!("Found spot for file {} at index {}", last_file_id, i);
                    new_index = Some(i);
                    space_size = *size;
                    break;
                }
            }
            if let Some(index) = new_index {
                if index > last_file_index {
                    if last_file_id == 0 {
                        break;
                    }
                    continue;
                }
                if last_file_size == space_size {
                    // println!("Swapping {} and {}", index, last_file_index);
                    self.blocks.swap(index, last_file_index);
                } else {
                    let unused_space = space_size - last_file_size;
                    self.blocks.swap(index, last_file_index);
                    self.blocks.get_mut(last_file_index).unwrap().1 = last_file_size;
                    self.blocks.insert(index + 1, (None, unused_space));
                }
            }
            if last_file_id == 0 {
                break;
            }
            // println!("{}", self);
        }
    }

    fn get_last_file(&self, file_id: usize) -> (usize, usize, usize) {
        let mut index = 1;
        loop {
            let x = self.blocks.get(self.blocks.len() - index).unwrap();
            match x {
                (Some(x), size) => {
                    if x < &file_id {
                        return (*x, *size, self.blocks.len() - index);
                    } else {
                        index += 1;
                    }
                }
                (None, _) => index += 1,
            }
        }
    }
    fn calculate_checksum(&self) -> usize {
        let mut total = 0;
        let mut i = 0;
        for (id_opt, size) in (&self).blocks.iter() {
            for _ in 0..*size {
                if let Some(id) = id_opt {
                    total += i * id;
                }
                i += 1;
            }
        }
        total
    }
}

impl fmt::Display for Disk2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut print_string: String = "".to_string();
        for (block_id_opt, size) in &self.blocks {
            for _ in 0..*size {
                match block_id_opt {
                    Some(c) => {
                        print_string.push_str(&format!("({})", c));
                    }
                    None => {
                        print_string.push('.');
                    }
                }
            }
        }
        write!(f, "{}", print_string)
    }
}

struct Disk {
    blocks: Vec<Option<usize>>,
}

impl Disk {
    fn compact_blocks(&self) -> Disk {
        let mut compacted_disk = Disk { blocks: vec![] };
        let mut last_index = 0;
        for (i, block) in (&self).blocks.iter().enumerate() {
            if i == self.blocks.len() - last_index {
                break;
            }
            // println!("{}", compacted_disk);
            match block {
                Some(c) => {
                    compacted_disk.blocks.push(Some(*c));
                }
                None => {
                    let index_last_block = self.find_last_block(last_index);
                    last_index = index_last_block;
                    let val = self
                        .blocks
                        .get(self.blocks.len() - index_last_block)
                        .unwrap();
                    compacted_disk.blocks.push(*val);
                }
            }
        }
        compacted_disk
    }

    fn find_last_block(&self, start_index: usize) -> usize {
        let mut index = start_index + 1;
        while let None = self.blocks.get(self.blocks.len() - index).unwrap() {
            index += 1;
        }
        index
    }

    fn calculate_checksum(&self) -> usize {
        let mut total = 0;
        for (i, block) in (&self).blocks.iter().enumerate() {
            total += i * block.unwrap();
        }
        total
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut print_string: String = "".to_string();
        for block in &self.blocks {
            match block {
                Some(c) => {
                    print_string.push_str(&format!("({})", c));
                }
                None => {
                    print_string.push('.');
                }
            }
        }
        write!(f, "{}", print_string)
    }
}

fn parse2(strings: &[String]) -> Disk2 {
    let mut v = vec![];
    let mut file_id = 0;
    for (i, c) in strings[0].chars().enumerate() {
        let c_num = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            v.push((Some(file_id), c_num));
            file_id += 1
        } else {
            v.push((None, c_num))
        }
    }
    Disk2 {
        blocks: v,
        max_file_id: file_id,
    }
}

fn parse(strings: &[String]) -> Disk {
    let mut v = vec![];
    let mut file_id = 0;
    for (i, c) in strings[0].chars().enumerate() {
        let c_num = c.to_digit(10).unwrap();
        for _ in 0..c_num {
            if i % 2 == 0 {
                // c is a file
                v.push(Some(file_id));
            } else {
                //c is empty space
                v.push(None);
            }
        }
        if i % 2 == 0 {
            file_id += 1
        }
    }
    Disk { blocks: v }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "2333133121414131402"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(1928, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2858, part2(&get_input()));
    }
}
