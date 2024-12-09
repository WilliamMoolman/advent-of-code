use std::collections::VecDeque;

advent_of_code::solution!(9);

#[derive(PartialEq)]
enum Block {
    Free,
    File(u64),
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk: VecDeque<(Block, u64)> = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            (
                if i % 2 == 0 {
                    Block::File(i as u64 / 2)
                } else {
                    Block::Free
                },
                c.to_digit(10).unwrap() as u64,
            )
        })
        .collect();
    let mut sum = 0;
    let mut idx = 0;
    while let Some((block, size)) = disk.pop_front() {
        if let Block::File(file_num) = block {
            for _ in 0..size {
                sum += idx * file_num;
                idx += 1;
            }
        } else {
            let mut remaining = size;
            while remaining > 0 {
                if disk.len() == 0 {
                    break;
                }
                // let back = &mut disk[disk.len() - 1];
                let disk_size = disk.len();
                match &mut disk[disk_size - 1] {
                    (Block::Free, _) => {
                        disk.pop_back();
                    }
                    (Block::File(back_num), ref mut back_size) => {
                        if *back_size == 0 {
                            disk.pop_back();
                        } else {
                            sum += idx * *back_num;
                            idx += 1;
                            remaining -= 1;
                            *back_size -= 1;
                        };
                    }
                }
            }
        }
    }
    Some(sum)
}

fn print_disk(disk: &VecDeque<(Block, u64)>) {
    disk.iter().for_each(|(b, size)| match b {
        Block::Free => {
            for _ in 0..*size {
                print!(".")
            }
        }
        Block::File(n) => {
            for _ in 0..*size {
                print!("{n}")
            }
        }
    });
    println!();
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut disk: VecDeque<(Block, u64)> = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            (
                if i % 2 == 0 {
                    Block::File(i as u64 / 2)
                } else {
                    Block::Free
                },
                c.to_digit(10).unwrap() as u64,
            )
        })
        .collect();

    // print_disk(&disk);

    let mut i = disk.len() - 1;
    while i > 0 {
        match disk[i] {
            (Block::Free, _) => (),
            (Block::File(file_num), size) => {
                for j in 0..i {
                    match disk[j] {
                        (Block::File(_), _) => continue,
                        (Block::Free, free_size) => {
                            if free_size < size {
                                continue;
                            } else if free_size == size {
                                disk[j] = (Block::File(file_num), size);
                                disk[i] = (Block::Free, size);
                            } else {
                                disk[j] = (Block::File(file_num), size);
                                disk[i] = (Block::Free, size);
                                disk.insert(j + 1, (Block::Free, free_size - size));
                                i += 1;
                            }
                            // print_disk(&disk);
                            break;
                        }
                    }
                }
            }
        }
        i -= 1;
    }
    // print_disk(&disk);

    let mut sum = 0;
    let mut idx = 0;
    while let Some((block, size)) = disk.pop_front() {
        match block {
            Block::File(file_num) => {
                for _ in 0..size {
                    sum += idx * file_num;
                    idx += 1;
                }
            }
            Block::Free => idx += size,
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
