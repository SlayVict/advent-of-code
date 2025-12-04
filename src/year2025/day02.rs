use crate::utils::{answers::Answer, iters::ChunkOps, parse::ParseOps};

type Range = [u32; 2];
type Pair = [u64; 2];

const FIRST: [Range; 5] = [[2, 1], [4, 2], [6, 3], [8, 4], [10, 5]];
const SECOND: [Range; 6] = [[3, 1], [5, 1], [6, 2], [7, 1], [9, 3], [10, 2]];
const THIRD: [Range; 2] = [[6, 1], [10, 1]];

pub fn part1(input: &str) -> Answer {
    let input = parse(input);

    //13108371860

    count(&FIRST, &input).into()
}

pub fn part2(input: &str) -> Answer {
    let input = parse(input);

    let result = count(&FIRST, &input) + count(&SECOND, &input) - count(&THIRD, &input);
    result.into()
}

fn count(ranges: &[Range], ids: &[Pair]) -> u64 {
    let mut result = 0;

    for &[digit_count, part_size] in ranges {
        let digits_magnitude = 10u64.pow(digit_count);
        let part_magnitude = 10u64.pow(part_size);

        let step = (digits_magnitude - 1) / (part_magnitude - 1);
        let start = step * (part_magnitude / 10);
        let end = step * (part_magnitude - 1);

        for &[from, to] in ids {
            let lower = from.next_multiple_of(step).max(start);
            let upper = to.min(end);

            if lower <= upper {
                let n = (upper - lower) / step;
                let triangular = n * (n + 1) / 2;
                result += lower * (n + 1) + step * triangular;
            }
        }
    }

    result
}

fn parse(input: &str) -> Vec<Pair> {
    let nums = input.iter_unsigned();
    nums.chunk::<2>().collect()
}
