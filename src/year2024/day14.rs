use crate::utils::{answers::Answer, grid::Grid, iters::ChunkOps, parse::ParseOps, point::Point};

struct Robot {
    pos: Point<i32>,
    vel: Point<i32>,
}

pub fn part1(input: &str) -> Answer {
    let robots = parse(input);
    let is_test = !robots.iter().any(|r| r.pos.x >= 11 || r.pos.y >= 7);
    let dimention = match is_test {
        true => (11, 7),
        false => (101, 103),
    };
    let mut quadrants = [0; 4];
    let middle = Point::new(dimention.0 / 2, dimention.1 / 2);
    for robot in robots.iter() {
        let mut pos = robot.pos;
        let vel = robot.vel;
        pos = pos + vel * 100;
        pos = Point::new(pos.x.rem_euclid(dimention.0), pos.y.rem_euclid(dimention.1));
        if pos.x == middle.x || pos.y == middle.y {
            continue;
        }
        let bools = (pos.x > middle.x, pos.y < middle.y);
        match bools {
            (false, false) => quadrants[0] += 1,
            (true, false) => quadrants[1] += 1,
            (true, true) => quadrants[2] += 1,
            (false, true) => quadrants[3] += 1,
        }
    }

    quadrants.iter().product::<i32>().into()
}

/*
* 01234567
* ...*........
* ..***.......
* .*****......
* *******.....
*/

fn find_tree(grid: &Grid<i32>) -> bool {
    let points: Vec<_> = (0..=3)
        .map(|y| ((-y)..=(y)).map(|x| Point::new(x, y)).collect::<Vec<_>>())
        .flatten()
        .collect();
    for x in 3..(grid.width - 3) {
        for y in 0..(grid.height - 4) {
            let mut flag = true;
            let point = Point::new(x, y);
            for &offset in points.iter() {
                flag &= grid.contains(point) && grid[point + offset] > 0;
                if !flag {
                    break;
                }
            }
            if flag {
                return true;
            }
        }
    }
    false
}

fn print_grid(grid: &Grid<i32>) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] > 0 {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
}

pub fn part2(input: &str) -> Answer {
    let mut robots = parse(input);

    let mut grid = Grid::new(101, 103, 0);

    for robot in robots.iter() {
        grid[robot.pos] += 1;
    }

    for seconds in 1..30000 {
        for robot in robots.iter() {
            let mut pos = robot.pos;
            let vel = robot.vel;
            pos = pos + vel * (seconds - 1);
            pos = Point::new(pos.x.rem_euclid(grid.width), pos.y.rem_euclid(grid.height));
            grid[pos] -= 1;
            pos = pos + vel;
            pos = Point::new(pos.x.rem_euclid(grid.width), pos.y.rem_euclid(grid.height));

            grid[pos] += 1;
        }

        if find_tree(&grid) {
            // print_grid(&grid);
            return seconds.into();
        }
    }

    1000000.into()
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .iter_signed::<i32>()
        .chunk::<4>()
        .map(|[rx, ry, vx, vy]| Robot {
            pos: Point { x: rx, y: ry },
            vel: Point { x: vx, y: vy },
        })
        .collect()
}
