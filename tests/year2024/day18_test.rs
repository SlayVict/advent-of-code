use aoc::year2024::day18::*;

const INPUT: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 22.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), "6,1".into());
}
