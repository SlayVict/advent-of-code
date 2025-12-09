use std::collections::HashMap;

use crate::utils::{answers::Answer, iters::ChunkOps, parse::ParseOps};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector3d {
    x: u64,
    y: u64,
    z: u64,
}

impl Vector3d {
    fn distance(&self, other: &Self) -> u64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
    weight: u64,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Vector3d>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(nodes: Vec<Vector3d>, edges: Vec<Edge>) -> Self {
        Self { nodes, edges }
    }

    fn find(&self, parent: &[usize], i: usize) -> usize {
        if parent[i] == i {
            i
        } else {
            self.find(parent, parent[i])
        }
    }

    fn union(
        &self,
        parent: &mut [usize],
        rank: &mut [usize],
        size: &mut [usize],
        x: usize,
        y: usize,
    ) -> usize {
        let root_x = self.find(parent, x);
        let root_y = self.find(parent, y);

        let size_x = size[root_x];
        let size_y = size[root_y];

        // if root_x != root_y {
        if rank[root_x] < rank[root_y] {
            parent[root_x] = root_y;
            size[root_y] += size_x;
            size[root_x] = 0;
            y
        } else if rank[root_x] > rank[root_y] {
            parent[root_y] = root_x;
            size[root_x] += size_y;
            size[root_y] = 0;
            x
        } else {
            parent[root_y] = root_x;
            rank[root_x] += 1;
            size[root_x] += size_y;
            size[root_y] = 0;
            x
        }
        // } else {
        //     root_x
        // }
    }
}

pub fn part1(input: &str) -> Answer {
    part1_sizeble(input, 1000)
}

pub fn part1_sizeble(input: &str, connection_count: usize) -> Answer {
    let graph = parse(input);
    let mut result = Vec::new();

    let mut parent: Vec<usize> = (0..graph.nodes.len()).collect();
    let mut rank: Vec<usize> = vec![0; graph.nodes.len()];
    let mut size: Vec<usize> = vec![1; graph.nodes.len()];

    for (i, edge) in (&graph.edges).iter().enumerate() {
        if graph.find(&parent, edge.from) != graph.find(&parent, edge.to) {
            let xy = graph.union(&mut parent, &mut rank, &mut size, edge.from, edge.to);
            result.push((xy, edge));
            if result.len() >= connection_count {
                break;
            }
        }
    }

    for (xy, edge) in &result {
        println!(
            "{:?} {:?} {:?}",
            edge, graph.nodes[edge.from], graph.nodes[edge.to]
        );
    }

    for (xy, edge) in &result {
        print!("{:>3}", graph.find(&parent, *xy));
    }
    println!();

    for i in 0..20 {
        print!("{:>3}", i);
    }
    println!();

    for i in 0..20 {
        print!("{:>3}", graph.find(&parent, i));
    }
    println!();

    for i in size {
        print!("{:>3}", i);
    }
    println!();

    // let mut map = HashMap::new();
    // for (root, _) in result {
    //     map.entry(root).and_modify(|v| *v += 1).or_insert(1);
    // }

    // let mut map: Vec<_> = map.into_iter().map(|(_, item)| item).collect();
    // map.sort_unstable();
    // // println!("{:?}", map);
    // map.iter()
    //     .rev()
    //     .take(3)
    //     .map(|&x| x + 1)
    //     .product::<u64>()
    //     .into()

    Answer::InProgress
}

pub fn part2(input: &str) -> Answer {
    Answer::InProgress
}

fn parse(input: &str) -> Graph {
    let nodes: Vec<_> = input
        .iter_unsigned()
        .chunk::<3>()
        .map(|chunk| Vector3d {
            x: chunk[0],
            y: chunk[1],
            z: chunk[2],
        })
        .collect();

    let mut edges: Vec<_> = nodes
        .iter()
        .enumerate()
        .flat_map(|(i, node)| {
            nodes
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, other)| Edge {
                    from: i,
                    to: j,
                    weight: node.distance(other),
                })
        })
        .collect();

    edges.sort_by_key(|edge| edge.weight);

    Graph::new(nodes, edges)
}
