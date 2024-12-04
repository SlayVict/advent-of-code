use crate::utils::answers::Answer;

fn solve(input: &Vec<Vec<char>>, leters: &[char], map: &Vec<Vec<(i32, i32)>>) -> i32 {
    let mut sum: i32 = 0;
    for (i, arr) in input.iter().enumerate() {
        for (j, c) in arr.iter().enumerate() {
            if leters[0] == *c {
                sum += map
                    .iter()
                    .filter(|&l| {
                        let shift: Vec<(i32, i32)> = l
                            .iter()
                            .map(|&(y, x)| (i as i32 + y, j as i32 + x))
                            .collect();
                        if shift.iter().any(|(y, x)| {
                            !(0..input.len() as i32).contains(&y)
                                || !(0..arr.len() as i32).contains(&x)
                        }) {
                            return false;
                        }

                        (0..leters.len())
                            .map(|index| {
                                input[shift[index].0 as usize][shift[index].1 as usize]
                                    == leters[index]
                            })
                            .fold(true, |a, b| a && b)
                    })
                    .count() as i32;
            }
        }
    }
    sum
}

pub fn part1(input: &str) -> Answer {
    let leters: [char; 4] = ['X', 'M', 'A', 'S'];

    let map: Vec<Vec<(i32, i32)>> = vec![
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (1, 1), (2, 2), (3, 3)],
        vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)],
        vec![(0, 0), (0, -1), (0, -2), (0, -3)],
        vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)],
        vec![(0, 0), (1, -1), (2, -2), (3, -3)],
        vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)],
    ];

    let input = parse(input);
    solve(&input, &leters, &map).into()
}

pub fn part2(input: &str) -> Answer {
    let leters: [char; 5] = ['A', 'M', 'M', 'S', 'S'];

    let map: Vec<Vec<(i32, i32)>> = vec![
        vec![(0, 0), (-1, -1), (1, -1), (1, 1), (-1, 1)],
        vec![(0, 0), (1, -1), (1, 1), (-1, -1), (-1, 1)],
        vec![(0, 0), (1, 1), (-1, 1), (-1, -1), (1, -1)],
        vec![(0, 0), (-1, -1), (-1, 1), (1, 1), (1, -1)],
    ];

    let input = parse(input);
    solve(&input, &leters, &map).into()
    // Answer::InProgress
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
