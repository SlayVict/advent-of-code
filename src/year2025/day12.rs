use crate::utils::{answers::Answer, iters::ChunkOps, parse::ParseOps};

pub fn part1(input: &str) -> Answer {
    input
        .iter_unsigned::<u32>()
        .skip(6)
        .chunk::<8>()
        .filter(|[w, h, presents @ ..]| (w / 3) * (h / 3) >= presents.iter().sum::<u32>())
        .count()
        .into()
}

pub fn part2(input: &str) -> Answer {
    Answer::String("No part 2 this time".to_string())
}
