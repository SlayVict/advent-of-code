use std::sync::Mutex;

use crate::utils::{answers::Answer, parse::ParseOps};
use rayon::prelude::*;

fn hash(mut n: usize) -> usize {
    n = (n ^ (n << 6)) & 0xffffff;
    n = (n ^ (n >> 5)) & 0xffffff;
    (n ^ (n << 11)) & 0xffffff
}

pub fn part1(input: &str) -> Answer {
    let numbers: Vec<usize> = input.iter_unsigned().collect();

    numbers
        .par_iter()
        .map(|&n| {
            let mut n = n;
            for _ in 0..2000 {
                n = hash(n);
            }
            n
        })
        .sum::<usize>()
        .into()
}

fn to_index(previous: usize, current: usize) -> usize {
    9 + current % 10 - previous % 10
}

pub fn part2(input: &str) -> Answer {
    let numbers: Vec<usize> = input.iter_unsigned().collect();
    let mut score = vec![0; 130321];
    let mut seen = vec![u16::MAX; 130321];

    numbers.iter().enumerate().for_each(|(id, number)| {
        let id = id as u16;

        let zeroth = *number;
        let first = hash(zeroth);
        let second = hash(first);
        let third = hash(second);

        let mut a;
        let mut b = to_index(zeroth, first);
        let mut c = to_index(first, second);
        let mut d = to_index(second, third);

        let mut number = third;
        let mut previous = third % 10;

        for _ in 3..2000 {
            number = hash(number);
            let price = number % 10;

            (a, b, c, d) = (b, c, d, 9 + price - previous);
            previous = price;
            let index = 6859 * a + 361 * b + 19 * c + d;

            if seen[index] != id {
                score[index] += price as u16;
                seen[index] = id;
            }
        }
    });

    score.iter().map(|&n| n).max().unwrap().into()
}

// fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
// }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_secret() {
        let mut num = 123;
        num = hash(num);
        assert_eq!(num, 15887950);
        num = hash(num);
        assert_eq!(num, 16495136);
        num = hash(num);
        assert_eq!(num, 527345);
        num = hash(num);
        assert_eq!(num, 704524);
        num = hash(num);
        assert_eq!(num, 1553684);
        num = hash(num);
        assert_eq!(num, 12683156);
        num = hash(num);
        assert_eq!(num, 11100544);
        num = hash(num);
        assert_eq!(num, 12249484);
        num = hash(num);
        assert_eq!(num, 7753432);
        num = hash(num);
        assert_eq!(num, 5908254);
    }
}
