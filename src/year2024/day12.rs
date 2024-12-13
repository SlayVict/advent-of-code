use crate::utils::{answers::Answer, grid::Grid, point::Point};

#[derive(Debug, Clone)]
struct Group {
    plots: Vec<Point<i32>>,
}

impl Group {
    fn new() -> Self {
        Self { plots: Vec::new() }
    }
    fn area(&self) -> u32 {
        self.plots.len() as u32
    }

    fn perimeter(&self, grid: &Grid<u8>) -> u32 {
        let mut per = 0;
        for &point in &self.plots {
            for direction in [
                Point::new(1, 0),
                Point::new(-1, 0),
                Point::new(0, 1),
                Point::new(0, -1),
            ] {
                if !(grid.contains(point + direction) && grid[point + direction] == grid[point]) {
                    per += 1;
                }
            }
        }
        per
    }

    fn sides(&self, grid: &Grid<u8>) -> u32 {
        let directions_clockvise = [
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(-1, 0),
            Point::new(0, -1),
            Point::new(1, 0),
        ];
        let mut siges = 0;
        for &point in &self.plots {
            for window in directions_clockvise.windows(2) {
                let d1 = window[0];
                let d2 = window[1];
                let is1 = if grid.contains(point + d1) && grid[point + d1] == grid[point] {
                    true
                } else {
                    false
                };
                let is2 = if grid.contains(point + d2) && grid[point + d2] == grid[point] {
                    true
                } else {
                    false
                };
                if !is1 && !is2 {
                    siges += 1;
                } else if is1 && is2 {
                    let diagonal = d1 + d2;
                    let is_diagonal = if grid.contains(point + diagonal)
                        && grid[point + diagonal] == grid[point]
                    {
                        true
                    } else {
                        false
                    };
                    if !is_diagonal {
                        siges += 1;
                    }
                }
            }
        }
        siges
    }
}

fn populate(
    map: &Grid<u8>,
    group_map: &mut Grid<Option<u32>>,
    group: &mut Vec<Group>,
    point: Point<i32>,
    id: u32,
) -> bool {
    if let Some(_) = group_map[point] {
        return false;
    }
    group_map[point] = Some(id);
    if group.len() == id as usize {
        group.push(Group::new());
    }
    group[id as usize].plots.push(point);
    for direction in [
        Point::new(1, 0),
        Point::new(-1, 0),
        Point::new(0, 1),
        Point::new(0, -1),
    ] {
        if map.contains(point + direction) && map[point + direction] == map[point] {
            populate(map, group_map, group, point + direction, id);
        }
    }
    true
}

#[warn(dead_code)]
fn print_grid(map: &Grid<Option<u32>>) {
    for y in 0..map.height {
        for x in 0..map.width {
            let point = Point::new(x, y);
            let c = if map[point].is_none() {
                "x"
            } else {
                &format!("{}", map[point].unwrap())
            };
            print!("{c} ");
        }
        println!();
    }
    println!();
}

pub fn part1(input: &str) -> Answer {
    let input = parse(input);
    let mut groups: Vec<Group> = Vec::new();
    let mut groups_grid = input.same_size_with(None);

    let mut group_id = 0;
    for y in 0..input.height {
        for x in 0..input.width {
            let point = Point::new(x, y);
            let res = populate(&input, &mut groups_grid, &mut groups, point, group_id);
            if res {
                group_id += 1;
            }
        }
    }
    // print_grid(&groups_grid);

    groups
        .iter()
        .map(|group| group.area() * group.perimeter(&input))
        .sum::<u32>()
        .into()
}

pub fn part2(input: &str) -> Answer {
    let input = parse(input);
    let mut groups: Vec<Group> = Vec::new();
    let mut groups_grid = input.same_size_with(None);

    let mut group_id = 0;
    for y in 0..input.height {
        for x in 0..input.width {
            let point = Point::new(x, y);
            let res = populate(&input, &mut groups_grid, &mut groups, point, group_id);
            if res {
                group_id += 1;
            }
        }
    }
    // print_grid(&groups_grid);

    groups
        .iter()
        .map(|group| group.area() * group.sides(&input))
        .sum::<u32>()
        .into()
}

fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}
