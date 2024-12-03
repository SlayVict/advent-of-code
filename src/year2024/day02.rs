use crate::utils::answers::Answer;

fn delta(l: i32, r: i32) -> i32 {
    let delta = r - l;
    if (1..=3).contains(&delta.abs()) {
        delta.signum()
    } else {
        0
    }
}

fn score(levels: &[i32]) -> i32 {
    levels.windows(2).map(|w| delta(w[0], w[1])).sum()
}

fn is_safe(levels: &[i32]) -> bool {
    let score = score(levels);
    let size = levels.len() as i32;

    if score.abs() == size - 1 {
        return true;
    }

    false
}

fn is_safe_2(levels: &[i32]) -> bool {
    let score = score(levels);
    let size = levels.len() as i32;

    if score.abs() == size - 1 {
        return true;
    }

    for i in 0..size as usize {
        let mut score = score;
        if i > 0 {
            score -= delta(levels[i - 1], levels[i]);
        }
        if i < size as usize - 1 {
            score -= delta(levels[i], levels[i + 1]);
        }
        if i > 0 && i < size as usize - 1 {
            score += delta(levels[i - 1], levels[i + 1]);
        }
        if score.abs() == size - 2 {
            return true;
        }
    }

    false
}

pub fn part1(input: &str) -> Answer {
    let report = parse(input);
    let result = report.iter().filter(|r| is_safe(r)).count();

    result.into()
}

pub fn part2(input: &str) -> Answer {
    let report = parse(input);
    let result = report.iter().filter(|r| is_safe_2(r)).count();

    result.into()
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = vec![];

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        reports.push(split.iter().map(|s| s.parse::<i32>().unwrap()).collect());
    }
    reports
}
