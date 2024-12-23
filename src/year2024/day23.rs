use std::collections::HashMap;

use crate::utils::answers::Answer;

pub fn part1(input: &str) -> Answer {
    let map = parse(input);

    let historians: Vec<_> = map.keys().filter(|k| k.starts_with("t")).collect();

    let mut groups = Vec::new();
    for (index, &&historian) in historians.iter().enumerate() {
        for i in 0..map[historian].len() {
            let teammate1 = map[historian][i];
            for j in i + 1..map[historian].len() {
                let teammate2 = map[historian][j];
                if map[teammate1].contains(&teammate2) {
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

    let mut groups = Vec::new();

    for (&pc1, connected) in map.iter() {
        let mut group = Vec::new();

        let mut queue = vec![pc1];
        while let Some(pc) = queue.pop() {
            let connections = &map[pc];
            if group.iter().all(|p| connections.contains(p)) {
                group.push(pc);
                queue.extend(connections);
            }
        }
        groups.push(group);
    }

    // for group in groups.iter() {
    //     println!("{:?}", group);
    // }

    let max_len = groups.iter().map(|g| g.len()).max().unwrap();
    let mut biggest = groups
        .iter()
        .filter(|g| g.len() == max_len)
        .collect::<Vec<_>>();

    let mut sorted = biggest[0].clone();
    sorted.sort();
    sorted.join(",").into()
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        let mut entry = map.entry(a).or_insert(Vec::new());
        if !entry.contains(&b) {
            entry.push(b);
        }

        entry = map.entry(b).or_insert(Vec::new());
        if !entry.contains(&a) {
            entry.push(a);
        }
    }

    map
}
