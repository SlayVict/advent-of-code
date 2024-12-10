use crate::utils::{answers::Answer, grid::Grid, point::Point};

fn score(point: Point, map: &Grid<u8>, scores: &mut Grid<Option<u32>>) -> u32 {
    if let Some(score) = scores[point] {
        return score;
    }
    if map[point] == b'9' {
        scores[point] = Some(1);
        return 1;
    }
    let mut price = 0;
    for direction in [
        Point::new(1, 0),
        Point::new(-1, 0),
        Point::new(0, 1),
        Point::new(0, -1),
    ] {
        if map.contains(point + direction)
            && map[point + direction].checked_sub(map[point]) == Some(1)
        {
            let s = score(point + direction, map, scores);
            price += s;
        }
    }
    scores[point] = Some(price);
    price
}

pub fn part1(input: &str) -> Answer {
    let map = parse(input);

    let mut final_score = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let point = Point::new(x as i32, y as i32);
            if map[point] != b'0' {
                continue;
            }
            let mut scores: Grid<Option<u32>> = map.same_size_with(None);
            let s = score(point, &map, &mut scores);
            final_score += scores
                .bytes
                .iter()
                .enumerate()
                .filter(|&(i, &s)| s.is_some() && map.bytes[i] == b'9')
                .count();
        }
    }

    final_score.into()
}

pub fn part2(input: &str) -> Answer {
    let map = parse(input);
    let mut scores: Grid<Option<u32>> = map.same_size_with(None);

    let mut final_score = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let point = Point::new(x as i32, y as i32);
            if map[point] != b'0' {
                continue;
            }
            let s = score(point, &map, &mut scores);
            final_score += s;
        }
    }

    final_score.into()
}

fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}
