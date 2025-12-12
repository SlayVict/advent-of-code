use crate::utils::answers::Answer;

type Input = Vec<Vec<usize>>;

pub fn part1(input: &str) -> Answer {
    paths(&parse(input), "you", "out").into()
}

pub fn part2(input: &str) -> Answer {
    let input = parse(input);
    let one =
        paths(&input, "svr", "fft") * paths(&input, "fft", "dac") * paths(&input, "dac", "out");
    let two =
        paths(&input, "svr", "dac") * paths(&input, "dac", "fft") * paths(&input, "fft", "out");
    (one + two).into()
}

fn paths(input: &Input, from: &str, to: &str) -> u64 {
    let mut cache = vec![u64::MAX; input.len()];
    dfs(input, &mut cache, to_index(from), to_index(to))
}

fn dfs(input: &Input, cache: &mut [u64], from: usize, to: usize) -> u64 {
    if from == to {
        return 1;
    }
    if cache[from] != u64::MAX {
        return cache[from];
    }

    let result = input[from]
        .iter()
        .map(|&next| dfs(input, cache, next, to))
        .sum();
    cache[from] = result;
    result
}

fn parse(input: &str) -> Input {
    let mut graph = vec![vec![]; 26 * 26 * 26];

    for line in input.lines() {
        let mut edges = line.split_ascii_whitespace();
        let from = edges.next().unwrap();
        graph[to_index(from)].extend(edges.map(to_index));
    }

    graph
}

fn to_index(s: &str) -> usize {
    s.bytes()
        .take(3)
        .fold(0, |acc, c| acc * 26 + usize::from(c - b'a'))
}
