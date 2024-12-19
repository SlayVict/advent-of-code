use std::collections::HashMap;

use crate::utils::answers::Answer;

struct Node {
    towel: bool,
    next: [usize; 26],
}

impl Node {
    fn new() -> Self {
        Self {
            towel: false,
            next: [0; 26],
        }
    }
}

fn check<'a>(trie: &[Node], seen: &mut HashMap<&'a [u8], u64>, pattern: &'a [u8]) -> u64 {
    if pattern.len() == 0 {
        return 1;
    }

    if let Some(&previous) = seen.get(pattern) {
        return previous;
    }

    let mut i = 0;
    let mut ways = 0;

    for depth in 0..pattern.len() {
        let j = to_index(pattern[depth]);
        i = trie[i].next[j];

        if i == 0 {
            break;
        }

        if trie[i].towel {
            ways += check(trie, seen, &pattern[depth + 1..]);
        }
    }

    seen.insert(pattern, ways);
    ways
}

pub fn part1(input: &str) -> Answer {
    let (towels, patterns) = parse(input);

    let seen = &mut HashMap::new();
    patterns
        .iter()
        .filter(|&&pattern| check(&towels, seen, pattern.as_bytes()) > 0)
        .count()
        .into()
}

pub fn part2(input: &str) -> Answer {
    let (towels, patterns) = parse(input);

    let seen = &mut HashMap::new();
    patterns
        .iter()
        .map(|&pattern| check(&towels, seen, pattern.as_bytes()))
        .sum::<u64>()
        .into()
}
#[inline]
fn to_index(b: u8) -> usize {
    (b - b'a') as usize
}
fn parse(input: &str) -> (Vec<Node>, Vec<&str>) {
    let mut trie = vec![Node::new()];
    let patterns: Vec<&str>;

    let (prefix, suffix) = input.split_once("\n\n").unwrap();

    // towels = split.next().unwrap().split(",").map(|s| s.trim()).collect();

    for towel in prefix.split(", ") {
        let mut i = 0;

        for j in towel.bytes().map(to_index) {
            if trie[i].next[j] == 0 {
                trie[i].next[j] = trie.len();
                i = trie.len();
                trie.push(Node::new());
            } else {
                i = trie[i].next[j];
            }
        }
        trie[i].towel = true;
    }

    patterns = suffix.lines().map(|s| s.trim()).collect();

    (trie, patterns)
}
