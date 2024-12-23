use aoc::year2024::day23::*;

const INPUT: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 7.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), "co,de,ka,ta".into());
}
