use std::collections::HashMap;

use crate::utils::{answers::Answer, iters::ChunkOps, parse::ParseOps};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector3d {
    x: usize,
    y: usize,
    z: usize,
}

impl Vector3d {
    fn distance(&self, other: &Self) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    parent: usize,
    size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
    weight: usize,
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

    fn find(&self, set: &mut [Node], mut x: usize) -> usize {
        while set[x].parent != x {
            let parent = set[x].parent;
            (x, set[x].parent) = (parent, set[parent].parent);
        }

        x
    }

    fn union(&self, set: &mut [Node], mut x: usize, mut y: usize) -> usize {
        x = self.find(set, x);
        y = self.find(set, y);

        if x != y {
            if set[x].size < set[y].size {
                (x, y) = (y, x);
            }

            set[y].parent = x;
            set[x].size += set[y].size;
            set[y].size = 0;
        }

        set[x].size
    }
}

pub fn part1(input: &str) -> Answer {
    part1_sizeble(input, 1000)
}

pub fn part1_sizeble(input: &str, limit: usize) -> Answer {
    let graph = parse(input);

    let mut set: Vec<Node> = (0..graph.nodes.len())
        .map(|i| Node { parent: i, size: 1 })
        .collect();

    for (i, edge) in (&graph.edges).iter().enumerate().take(limit) {
        if graph.find(&mut set, edge.from) != graph.find(&mut set, edge.to) {
            graph.union(&mut set, edge.from, edge.to);
        }
    }

    set.sort_unstable_by_key(|node| node.size);
    set.iter()
        .rev()
        .take(3)
        .map(|node| node.size)
        .product::<usize>()
        .into()
}

pub fn part2(input: &str) -> Answer {
    Answer::InProgress
}

fn parse(input: &str) -> Graph {
    let nodes: Vec<_> = input
        .iter_unsigned::<usize>()
        .chunk::<3>()
        .map(|chunk| Vector3d {
            x: chunk[0],
            y: chunk[1],
            z: chunk[2],
        })
        .collect();

    let mut edges: Vec<_> = Vec::new();

    for (i, node) in nodes.iter().enumerate() {
        for (j, other) in nodes.iter().enumerate().skip(i + 1) {
            edges.push(Edge {
                from: i,
                to: j,
                weight: node.distance(other),
            });
        }
    }

    edges.sort_unstable_by_key(|edge| edge.weight);

    Graph::new(nodes, edges)
}
