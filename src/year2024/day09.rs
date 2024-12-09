use crate::utils::answers::Answer;

pub fn part1(input: &str) -> Answer {
    let mut input = parse(input);
    let mut index: u64 = 0;
    let mut left: u64 = 0;
    let mut left_index = 0 as usize;
    let mut right: u64 = (input.len() - 1) as u64 / 2;
    let mut right_index = input.len() - 2 + input.len() % 2;
    let mut sum: u64 = 0;

    let mut accounted = 0;

    // while left < right {
    //     if let Some(l) = input[left as usize] {
    //         sum += l as u64 * left;
    //         // print!("{l} ");
    //         left += 1;
    //     } else if let Some(r) = input[right as usize - 1] {
    //         right -= 1;
    //         sum += r as u64 * left;
    //         // print!("{r} ");
    //         left += 1;
    //     } else {
    //         right -= 1;
    //     }
    // }
    // println!();

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

pub fn part2(input: &str) -> Answer {
    let mut input = parse_2(input);
    let size = input.len();
    let mut moved = Vec::new();

    let mut prev_index = 0;
    loop {
        let moved_count = moved.len();
        println!("{moved_count}/{size}");
        let first = input
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, &item)| {
                if let DriveSpace::File((width, id)) = item {
                    !moved.contains(&id)
                } else {
                    false
                }
            })
            .next();
        let Some((file_index, &file)) = first else {
            break;
        };
        let DriveSpace::File((file_width, id)) = file else {
            panic!()
        };
        moved.push(id);

        let first_free = input[..file_index]
            .iter()
            .enumerate()
            .filter(|(_, &space)| match space {
                DriveSpace::File(_) => false,
                DriveSpace::Free(free_width) => free_width >= file_width,
            })
            .next();

        if let Some((free_index, &free_space)) = first_free {
            if free_space.width() > file_width {
                input[free_index] = DriveSpace::Free(free_space.width() - file_width);
            } else {
                input.remove(free_index);
            }
            let tmp = input.iter().position(|i| *i == file).unwrap();
            input.remove(tmp);
            input.insert(tmp, DriveSpace::Free(file.width()));

            input.insert(free_index, file);
        }
    }

    // print_drive(&input);
    let mut sum = 0u64;
    let mut index = 0u64;
    for space in input {
        let c = match space {
            DriveSpace::File((_, id)) => id,
            DriveSpace::Free(_) => 0,
        };
        sum += (0..space.width())
            .map(|i| (c as u64 * (index + i as u64)) as u64)
            .sum::<u64>();
        index += space.width() as u64;
    }

    sum.into()
}

fn parse(input: &str) -> Vec<u8> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn parse_2(input: &str) -> Vec<DriveSpace> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let width = c.to_digit(10).unwrap();
            match i % 2 {
                0 => DriveSpace::File((width, i as u32 / 2)),
                _ => DriveSpace::Free(width),
            }
        })
        .filter(|space| match space {
            DriveSpace::File((width, _)) => *width > 0,
            DriveSpace::Free(width) => *width > 0,
        })
        .collect()
}
