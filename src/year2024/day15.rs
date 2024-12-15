use crate::utils::{answers::Answer, direction::Direction, grid::Grid, point::Point};

pub fn part1(input: &str) -> Answer {
    Answer::InProgress
}

pub fn part2(input: &str) -> Answer {
    Answer::InProgress
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Crate,
    Empty,
    Robot,
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            '.' => Self::Empty,
            '0' => Self::,
            '#' => Self::Empty,
            '@' => Self::Empty,
            _ => panic!("Unknown map type"),
        }
    }
}

fn parse(input: &str) -> (Grid<Cell>, Vec<Direction>) {
    let split = input.split_whitespace().collect::<Vec<_>>();
    let map_char = Grid::parse(split[0]);
    let mut map = map_char.same_size_with(Cell::Empty);

    let directions = split[1]
        .lines()
        .map(|line| line.chars().map(|c| Direction::from(c)).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();

    for x in 0..(map_char.width) {
        for y in 0..(map_char.height) {
            let point = Point::new(x, y);
            map[point] = map_char[point].into();
        }
    }
    (map, directions)
}
