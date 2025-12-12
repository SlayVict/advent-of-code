use aoc::year2025::day11::*;

const INPUT1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

const INPUT2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT1), 5.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT2), 2.into());
}
