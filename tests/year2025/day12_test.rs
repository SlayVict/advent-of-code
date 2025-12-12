use aoc::year2025::day12::*;

const INPUT: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 2.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), 33.into());
}
