use crate::utils::{
    answers::Answer,
    direction::{self, Direction},
    grid::Grid,
    point::Point,
};

fn turn(map: &mut Grid<Cell>, robot_position: Point<i32>, direction: Direction) -> Point<i32> {
    let next = robot_position + direction.into();
    match map[next] {
        Cell::Wall => robot_position,
        Cell::Empty => {
            map[robot_position] = Cell::Empty;
            map[next] = Cell::Robot;
            next
        }
        Cell::Robot => robot_position,
        Cell::Crate => {
            let mut i = 2;
            loop {
                let future = robot_position + Point::from(direction) * i;
                match map[future] {
                    Cell::Wall => return robot_position,
                    Cell::Crate => (),
                    Cell::Robot => (),
                    Cell::Empty => {
                        map[future] = Cell::Crate;
                        map[robot_position] = Cell::Empty;
                        map[next] = Cell::Robot;
                        return next;
                    }
                }
                i += 1;
            }
        }
    }
}

fn print_map(map: &Grid<Cell>) {
    for y in 0..map.height {
        for x in 0..map.width {
            let point = Point::new(x, y);
            let c: char = map[point].into();
            print!("{c}");
        }
        println!();
    }
}

pub fn part1(input: &str) -> Answer {
    let (mut map, directions) = parse(input);
    let mut robot_position = map.find(Cell::Robot).unwrap();

    for direction in directions {
        robot_position = turn(&mut map, robot_position, direction);
    }

    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let point = Point::new(x, y);
            if map[point] == Cell::Crate {
                sum += y * 100 + x;
            }
        }
    }

    sum.into()
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

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Wall => '#',
            Cell::Crate => 'O',
            Cell::Empty => '.',
            Cell::Robot => '@',
        }
    }
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'O' => Self::Crate,
            b'#' => Self::Wall,
            b'@' => Self::Robot,
            c => panic!("Unknown map type {}", c as char),
        }
    }
}

fn parse(input: &str) -> (Grid<Cell>, Vec<Direction>) {
    let split = input.split("\n\n").collect::<Vec<_>>();
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
