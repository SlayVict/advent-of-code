use crate::utils::{
    answers::Answer,
    direction::{self, Direction},
    grid::Grid,
    point::Point,
};

fn can_big_crate_push(map: &Grid<Cell>, position: Point<i32>, direction: Direction) -> bool {
    let part = map[position];
    if part != Cell::BigCrateLeft && part != Cell::BigCrateRight {
        return false;
    }

    let second_position: Point<i32>;
    if part == Cell::BigCrateLeft {
        second_position = position + Direction::Right.into();
    } else {
        second_position = position + Direction::Left.into();
    }

    let mut nexts = vec![position, second_position];
    nexts.iter_mut().for_each(|next| *next += direction.into());
    nexts.iter().all(|next| {
        let next_cell = map[*next];
        if let Direction::Up | Direction::Down = direction {
            match next_cell {
                Cell::Empty => true,
                Cell::BigCrateLeft | Cell::BigCrateRight => {
                    can_big_crate_push(map, *next, direction)
                }
                _ => false,
            }
        } else {
            let mut i = 2;
            loop {
                let future = position + Point::from(direction) * i;
                match map[future] {
                    Cell::Empty => return true,
                    Cell::BigCrateLeft | Cell::BigCrateRight => {}
                    _ => return false,
                }
                i += 2;
            }
        }
    })
}

fn _push(
    map: &mut Grid<Cell>,
    position: Point<i32>,
    direction: Direction,
    can_push: Option<bool>,
) -> Point<i32> {
    let next = position + direction.into();
    let original_cell = map[position];
    if original_cell == Cell::Empty || original_cell == Cell::Wall {
        return position;
    }
    // println!(
    //     "{position:?}={original_cell:?};\tnext:{next:?}={:?}; direction: {direction:?}",
    //     map[next]
    // );
    match map[next] {
        Cell::Empty => {
            map[position] = Cell::Empty;
            map[next] = original_cell;
            next
        }
        Cell::Crate => {
            let mut i = 2;
            loop {
                let future = position + Point::from(direction) * i;
                match map[future] {
                    Cell::Wall => return position,
                    Cell::Empty => {
                        map[future] = Cell::Crate;
                        map[position] = Cell::Empty;
                        map[next] = original_cell;
                        return next;
                    }
                    _ => (),
                }
                i += 1;
            }
        }
        Cell::BigCrateLeft | Cell::BigCrateRight => {
            let can_push = match can_push {
                Some(v) => v,
                None => can_big_crate_push(map, next, direction),
            };
            // println!("can_push: {can_push}");

            if can_push {
                let second_position: Point<i32>;
                if map[next] == Cell::BigCrateLeft {
                    second_position = next + Direction::Right.into();
                } else {
                    second_position = next + Direction::Left.into();
                }

                let nexts = if let Direction::Up | Direction::Down = direction {
                    vec![next, second_position]
                } else {
                    vec![next]
                };
                for next in nexts {
                    _push(map, next, direction, Some(can_push));
                }

                map[position] = Cell::Empty;
                map[next] = original_cell;
                next
            } else {
                position
            }
        }
        _ => position,
    }
}

fn push(map: &mut Grid<Cell>, position: Point<i32>, direction: Direction) -> Point<i32> {
    _push(map, position, direction, None)
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
        robot_position = push(&mut map, robot_position, direction);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Crate,
    Empty,
    Robot,
    BigCrateLeft,
    BigCrateRight,
}

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Wall => '#',
            Cell::Crate => 'O',
            Cell::Empty => '.',
            Cell::Robot => '@',
            Cell::BigCrateLeft => '[',
            Cell::BigCrateRight => ']',
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
            b'[' => Self::BigCrateLeft,
            b']' => Self::BigCrateRight,
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

pub fn part2(input: &str) -> Answer {
    let (mut map, directions) = parse2(input);
    let mut robot_position = map.find(Cell::Robot).unwrap();

    // print_map(&map);
    // println!("\n");

    for direction in directions {
        robot_position = push(&mut map, robot_position, direction);
        // println!("{direction:?}");
        // print_map(&map);
    }

    // println!("\n");
    // print_map(&map);
    // println!("\n");

    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let point = Point::new(x, y);
            if map[point] == Cell::BigCrateLeft {
                sum += y * 100 + x;
            }
        }
    }

    sum.into()
}

fn parse2(input: &str) -> (Grid<Cell>, Vec<Direction>) {
    let split = input.split("\n\n").collect::<Vec<_>>();
    let map_char = Grid::parse(split[0]);
    let mut map = Grid::new(map_char.width * 2, map_char.height, Cell::Empty);

    let directions = split[1]
        .lines()
        .map(|line| line.chars().map(|c| Direction::from(c)).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();

    for x in 0..(map_char.width) {
        for y in 0..(map_char.height) {
            let point1 = Point::new(x, y);
            let point_l = Point::new(x * 2, y);
            let point_r = Point::new(x * 2 + 1, y);
            let cell: Cell = map_char[point1].into();
            map[point_l] = cell;
            map[point_r] = cell;
            if cell == Cell::Crate {
                map[point_l] = Cell::BigCrateLeft;
                map[point_r] = Cell::BigCrateRight;
            } else if cell == Cell::Robot {
                map[point_l] = Cell::Robot;
                map[point_r] = Cell::Empty;
            }
        }
    }
    (map, directions)
}
