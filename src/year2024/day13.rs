use crate::utils::{answers::Answer, iters::ChunkOps, parse::ParseOps, point::Point};

#[derive(Debug)]
struct Game {
    button_a: Point<i64>,
    button_b: Point<i64>,
    target: Point<i64>,
}

fn check(game: &Game) -> Option<i64> {
    let x0 = game.button_a.x;
    let y0 = game.button_a.y;
    let x1 = game.button_b.x;
    let y1 = game.button_b.y;
    let xt = game.target.x;
    let yt = game.target.y;
    let b_numerator = xt * y0 - x0 * yt;
    let b_denominator = x1 * y0 - y1 * x0;
    if b_numerator % b_denominator != 0 {
        return None;
    }
    let b = b_numerator / b_denominator;

    let a_numerator = yt - y1 * b;
    let a_denom = y0;
    if a_numerator % a_denom != 0 {
        return None;
    }
    let a = a_numerator / a_denom;

    Some(a * 3 + b)
}

pub fn part1(input: &str) -> Answer {
    let games = parse(input);
    games
        .iter()
        .filter_map(|game| check(&game))
        .sum::<i64>()
        .into()
}

pub fn part2(input: &str) -> Answer {
    let games = parse(input);
    let offset = Point::<i64>::new(10000000000000, 10000000000000);
    games
        .iter()
        .map(|game| Game {
            button_a: game.button_a,
            button_b: game.button_b,
            target: game.target + offset,
        })
        .filter_map(|game| check(&game))
        .sum::<i64>()
        .into()
}

fn parse(input: &str) -> Vec<Game> {
    let nums = input.iter_signed::<i64>().chunk::<6>();
    nums.map(|chunk| Game {
        button_a: Point::new(chunk[0], chunk[1]),
        button_b: Point::new(chunk[2], chunk[3]),
        target: Point::new(chunk[4], chunk[5]),
    })
    .collect()
}
