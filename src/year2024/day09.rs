use std::{cmp::Reverse, collections::BinaryHeap, error::Error};

use crate::utils::answers::Answer;

pub fn part1(input: &str) -> Answer {
    let mut input: Vec<_> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut index: u64 = 0;
    let mut left: u64 = 0;
    let mut left_index = 0 as usize;
    let mut right: u64 = (input.len() - 1) as u64 / 2;
    let mut right_index = input.len() - 2 + input.len() % 2;
    let mut sum: u64 = 0;

    let mut accounted = 0;

    while left_index <= right_index {
        if left_index % 2 == 0 {
            if input[left_index] > 0 {
                sum += index * left as u64;
                // print!("{left} ");
                index += 1;
                input[left_index] -= 1;
            } else {
                left_index += 1;
                left += 1;
            }
        } else {
            if input[left_index] <= 0 {
                left_index += 1;
                continue;
            }
            if input[right_index] > 0 {
                sum += index * right as u64;
                // print!("{right} ");
                index += 1;
                input[right_index] -= 1;
                input[left_index] -= 1;
            } else {
                right -= 1;
                right_index -= 2;
            }
        }
    }

    sum.into()
}

#[derive(PartialEq, Clone, Copy)]
enum DriveSpace {
    File((u32, u32)), // (width, id)
    Free(u32),        // width
}

impl DriveSpace {
    fn width(&self) -> u32 {
        match *self {
            DriveSpace::File((width, _)) => width,
            DriveSpace::Free(width) => width,
        }
    }
}

fn print_drive(input: &Vec<DriveSpace>) {
    for space in input {
        let c = match space {
            DriveSpace::File((_, id)) => format!("{id}").chars().next().unwrap(),
            DriveSpace::Free(_) => '.',
        };
        for _ in 0..space.width() {
            print!("{c} ");
        }
    }
    println!()
}

#[derive(Debug, Clone)]
struct SizeError;

// impl Error for SizeError {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct File {
    start: u64, // inclusive
    end: u64,   // inclusive
    id: u64,
}

impl File {
    fn new(start: u64, end: u64, id: u64) -> Result<Self, SizeError> {
        if start <= end {
            Ok(Self { start, end, id })
        } else {
            Err(SizeError)
        }
    }
    fn width(&self) -> u64 {
        self.end - self.start + 1
    }
    fn checksum(&self) -> u64 {
        (self.start..=self.end).sum::<u64>() * self.id
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Free {
    start: u64,
    end: u64,
}

impl Free {
    fn new(start: u64, end: u64) -> Result<Self, SizeError> {
        if start <= end {
            Ok(Self { start, end })
        } else {
            Err(SizeError)
        }
    }

    fn width(&self) -> u64 {
        if self.end < self.start {
            return 0;
        }
        self.end - self.start + 1
    }
}

impl Ord for Free {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Free {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.start.cmp(&other.start))
    }
}

#[derive(Debug)]
struct Drive {
    files: Vec<File>,
    free: Vec<Free>,
}

fn display_files(files: &Vec<File>) {
    let mut files = files.clone();
    files.sort_by_key(|&f| f.start);
    let mut file_index = 0usize;
    let mut index = 0usize;

    let mut id = 0;
    while file_index < files.len() {
        if index <= files[file_index].end as usize && index >= files[file_index].start as usize {
            id = files[file_index].id;
            print!("{id}");
        } else {
            print!(".");
        }
        index += 1;
        if index > files[file_index].end as usize {
            file_index += 1;
        }
    }
    println!()
}

pub fn part2(input: &str) -> Answer {
    /* target 6347435485773 */
    let mut drive = parse(input);

    let mut last_free = [Some(0); 9];

    // display_files(&drive.files);

    for i in (0..drive.files.len()).rev() {
        let mut file = drive.files[i];
        let width = file.width();

        let from = last_free[width as usize - 1];
        let Some(from) = from else { continue };
        let free_index = (from..drive.free.len())
            .filter_map(|index| {
                let free = drive.free[index];
                let free_width = free.width();
                if free_width >= width {
                    Some(index)
                } else {
                    None
                }
            })
            .next();

        last_free[width as usize - 1] = free_index;

        if let Some(index) = free_index {
            let mut free = drive.free[index];
            if free.start > file.start {
                continue;
            }
            file.start = free.start;
            file.end = file.start + width - 1;
            drive.files[i] = file;
            free.start = file.end + 1;
            drive.free[index] = free;
        }
    }

    // display_files(&drive.files);
    drive
        .files
        .iter()
        .map(|&f| f.checksum())
        .sum::<u64>()
        .into()
}

fn parse(input: &str) -> Drive {
    let mut free = Vec::new();
    let mut files = Vec::new();
    let iter = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8);

    let mut index = 0;
    for (i, width) in iter.enumerate() {
        if width > 0 {
            if i % 2 == 0 {
                files.push(File::new(index, index + width as u64 - 1, i as u64 / 2).unwrap());
            } else {
                free.push(Free::new(index, index + width as u64 - 1).unwrap());
            }
        }
        index += width as u64;
    }

    Drive { files, free }
}
