use std::collections::HashMap;

use crate::utils::answers::Answer;

pub fn part1(input: &str) -> Answer {
    let map = parse(input);

    let historians: Vec<_> = map.keys().filter(|k| k.starts_with("t")).collect();

    let mut groups = Vec::new();
    for (index, &&historian) in historians.iter().enumerate() {
        for i in 0..map[historian].1.len() {
            let teammate1 = map[historian].1[i];
            for j in i + 1..map[historian].1.len() {
                let teammate2 = map[historian].1[j];
                if map[teammate1].1.contains(&teammate2) {
                    if teammate1.starts_with("t") {
                        if index < historians.iter().position(|&&h| h == teammate1).unwrap() {
                            continue;
                        }
                    }
                    if teammate2.starts_with("t") {
                        if index < historians.iter().position(|&&h| h == teammate2).unwrap() {
                            continue;
                        }
                    }
                    groups.push(vec![historian, teammate1, teammate2]);
                }
            }
        }
    }

    // for group in groups {
    //     println!("{:?}", group);
    // }

    groups.len().into()
}

pub fn part2(input: &str) -> Answer {
    let map = parse(input);

    let mut lan = Vec::new();

    for (i, &pc1) in map.keys().enumerate() {
        let mut group = Vec::new();

        let mut queue = vec![pc1];
        while let Some(pc) = queue.pop() {
            let (j, connections) = &map[pc];
            if group.iter().all(|p| connections.contains(p)) {
                group.push(pc);
                queue.extend(connections);
            }
        }

        if group.len() > lan.len() {
            lan = group;
        }
    }

    lan.sort();
    lan.join(",").into()
}

fn parse(input: &str) -> HashMap<&str, (usize, Vec<&str>)> {
    let mut map = HashMap::new();

    let mut len: usize;
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        len = map.len();
        let mut entry = map.entry(a).or_insert((len, Vec::new()));
        if !entry.1.contains(&b) {
            entry.1.push(b);
        }

        len = map.len();
        entry = map.entry(b).or_insert((len, Vec::new()));
        if !entry.1.contains(&a) {
            entry.1.push(a);
        }
    }

    map
}
