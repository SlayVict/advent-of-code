use crate::utils::answers::Answer;

pub fn part1(input: &str) -> Answer {
    let input = parse(input);

    let mut dial = 50;
    let mut count = 0;

    for i in 0..input.len() {
        dial = (dial + input[i]).rem_euclid(100);
        count += (dial == 0) as i32;
    }

    count.into()
}

pub fn part2(input: &str) -> Answer {
    let mut input = parse(input);

    let mut dial = 50;
    let mut count = 0;

    for i in 0..input.len() {
        if input[i] >= 0 {
            count += (dial + input[i]) / 100;
        } else {
            let reverse = (100 - dial) % 100;
            count += (reverse - input[i]) / 100;
        }

        let sum = dial + input[i];
        dial = sum.rem_euclid(100);
    }

    count.into()
}

fn parse(input: &str) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();

    for line in input.lines() {
        let direction = line.trim();

        if direction.starts_with('R') {
            let distance = direction[1..].parse().unwrap();
            nums.push(distance);
        } else if direction.starts_with('L') {
            let distance: i32 = direction[1..].parse().unwrap();
            nums.push(-distance);
        }
    }

    nums
}
