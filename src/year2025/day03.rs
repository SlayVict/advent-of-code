use crate::utils::{answers::Answer, grid::Grid, parse::ParseByte, point::Point};

pub fn part1(input: &str) -> Answer {
    let grid = parse(input);

    let mut result = 0;

    for i in 0..grid.height {
        let width = grid.width;

        let mut buff: Vec<[u8; 2]> = vec![[0, 0]; (width - 1) as usize];

        for j in 0..width - 1 {
            let point = Point::new(j, i);
            let jolt = grid[point].to_decimal();

            buff[j as usize][0] = buff[(j - 1).max(0) as usize][0].max(jolt);
        }

        let last = Point::new(width - 1, i);
        let jolt = grid[last].to_decimal();
        buff[width as usize - 2][1] = jolt;

        for j in (1..width - 1).rev() {
            let point = Point::new(j, i);
            let jolt = grid[point].to_decimal();

            buff[j as usize - 1][1] = buff[(j).min(width - 1) as usize][1].max(jolt);
        }

        result += buff
            .iter()
            .map(|pair| pair[0] as u32 * 10 + pair[1] as u32)
            .max()
            .unwrap();
    }

    result.into()
}

pub fn part2(input: &str) -> Answer {
    Answer::InProgress
}

fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}
