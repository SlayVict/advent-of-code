use aoc::year2024::day12::*;

const INPUT1: &str = "\
AAAA
BBCD
BBCC
EEEC";

const INPUT2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

const INPUT3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

const INPUT4: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

const INPUT5: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

#[test]
fn part1_test1() {
    assert_eq!(part1(&INPUT1), 140.into());
}
#[test]
fn part1_test2() {
    assert_eq!(part1(&INPUT2), 772.into());
}
#[test]
fn part1_test3() {
    assert_eq!(part1(&INPUT3), 1930.into());
}

#[test]
fn part2_test1() {
    assert_eq!(part2(&INPUT1), 80.into());
}
#[test]
fn part2_test2() {
    assert_eq!(part2(&INPUT2), 436.into());
}
#[test]
fn part2_test3() {
    assert_eq!(part2(&INPUT3), 1206.into());
}
#[test]
fn part2_test4() {
    assert_eq!(part2(&INPUT4), 236.into());
}
#[test]
fn part2_test5() {
    assert_eq!(part2(&INPUT5), 368.into());
}
