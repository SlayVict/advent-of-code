use core::panic;

use crate::utils::{answers::Answer, iters::ChunkOps, parse::ParseOps};

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

enum Instrcuction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Instrcuction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Unknown value {}", value),
        }
    }
}

fn interpriter(reg: &mut Registers, program: &Vec<u8>) -> Vec<u8> {
    let mut pointer = 0;
    let mut output = vec![];

    // println!("{reg:?}");

    let combo = |value: u8, reg: &Registers| match value {
        v @ 0..=3 => v as u64,
        4 => reg.a,
        5 => reg.b,
        6 => reg.c,
        _ => panic!("Unknown value {}", value),
    };

    while pointer < program.len() {
        let mut move_pointer = true;
        match program[pointer].into() {
            Instrcuction::Adv => {
                let numerator = reg.a;
                let denum = 1 << combo(program[pointer + 1], &reg);
                reg.a = numerator / denum;
            }
            Instrcuction::Bxl => {
                reg.b ^= program[pointer + 1] as u64;
            }
            Instrcuction::Bst => {
                reg.b = combo(program[pointer + 1], &reg) % 8;
            }
            Instrcuction::Jnz => {
                if reg.a != 0 {
                    pointer = program[pointer + 1] as usize;
                    move_pointer = false;
                }
            }
            Instrcuction::Bxc => {
                reg.b = reg.b ^ reg.c;
            }
            Instrcuction::Out => {
                let operator = (combo(program[pointer + 1], &reg) % 8) as u8;
                output.push(operator);
            }
            Instrcuction::Bdv => {
                let numerator = reg.a;
                let denum = 1 << combo(program[pointer + 1], &reg);
                reg.b = numerator / denum;
            }
            Instrcuction::Cdv => {
                let numerator = reg.a;
                let denum = 1 << combo(program[pointer + 1], &reg);
                reg.c = numerator / denum;
            }
        }
        if move_pointer {
            pointer += 2;
        }
    }

    output
}

pub fn part1(input: &str) -> Answer {
    let (mut reg, program) = parse(input);

    interpriter(&mut reg, &program)
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(",")
        .into()
}

// 109019930331546
pub fn part2(input: &str) -> Answer {
    let (mut reg, program) = parse(input);

    let mut valid = vec![0];

    for &target in program.iter().rev() {
        let mut next = Vec::new();

        for v in valid {
            for n in 0..8 {
                let a = (v << 3) | n;
                let mut reg = Registers {
                    a,
                    b: reg.b,
                    c: reg.c,
                };

                let res = interpriter(&mut reg, &program);
                if res[0] == target {
                    next.push(a);
                }
            }
        }

        valid = next;
    }
    (*valid.iter().min().unwrap()).into()
}

fn parse(input: &str) -> (Registers, Vec<u8>) {
    let split = input.split("\n\n").collect::<Vec<_>>();

    let registers_iter = split[0].iter_unsigned::<u64>().collect::<Vec<_>>();
    let registers = Registers {
        a: registers_iter[0],
        b: registers_iter[1],
        c: registers_iter[2],
    };

    let program = split[1].iter_unsigned::<u8>().collect::<Vec<_>>();

    (registers, program)
}
