use crate::utils::{answers::Answer, grid::Grid, point::Point};

fn pave(
    point: Point<i32>,
    map: &Grid<u8>,
    met_vec: &mut Vec<Vec<Option<Vec<Point<i32>>>>>,
) -> Vec<Point<i32>> {
    if let Some(met) = met_vec[point.y as usize][point.x as usize].clone() {
        return met;
    }
    if map[point] == b'9' {
        met_vec[point.y as usize][point.x as usize] = Some(vec![point]);
        return met_vec[point.y as usize][point.x as usize].clone().unwrap();
    }

    let mut price = Vec::new();
    for direction in [
        Point::new(1, 0),
        Point::new(-1, 0),
        Point::new(0, 1),
        Point::new(0, -1),
    ] {
        if map.contains(point + direction)
            && map[point + direction].checked_sub(map[point]) == Some(1)
        {
            let s = pave(point + direction, map, met_vec);
            for p in s.iter() {
                if !price.contains(p) {
                    price.push(*p);
                }
            }
        }
    }
    met_vec[point.y as usize][point.x as usize] = Some(price.clone());
    price
}

pub fn part1(input: &str) -> Answer {
    let map = parse(input);
    let mut met: Vec<Vec<Option<Vec<Point<i32>>>>> =
        vec![vec![None; map.width as usize]; map.height as usize];

    let mut final_score = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let point = Point::new(x as i32, y as i32);
            if map[point] != b'0' {
                continue;
            }
            let m = pave(point, &map, &mut met);
            final_score += m.len();
        }
    }

    final_score.into()
}

fn score(point: Point<i32>, map: &Grid<u8>, scores: &mut Grid<Option<u32>>) -> u32 {
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
