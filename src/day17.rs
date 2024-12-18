use anyhow::Result;
use itertools::Itertools;
use std::time::{Duration, Instant};

use crate::input::ints;

struct Machine {
    reg: [i128; 3],
    ip: usize,
}

impl Machine {
    fn new(a: i128, b: i128, c: i128) -> Self {
        Self {
            reg: [a, b, c],
            ip: 0,
        }
    }

    fn run(&mut self, prog: &[u8], search: bool) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        loop {
            if self.ip >= prog.len() {
                break;
            }
            let opcode = prog[self.ip];
            let operand = prog[self.ip + 1];
            match opcode {
                0 => {
                    self.reg[0] = self.adv(operand);
                }
                1 => {
                    self.reg[1] = operand as i128 ^ self.b();
                }
                2 => {
                    self.reg[1] = self.combo(operand) % 8;
                }
                3 => {
                    if self.a() != 0 {
                        self.ip = operand as usize;
                        continue;
                    }
                }
                4 => {
                    self.reg[1] = self.b() ^ self.c();
                }
                5 => {
                    let val = self.combo(operand) % 8;
                    out.push(val.try_into().unwrap());
                    if search && !prog.starts_with(&out) {
                        break;
                    }
                }
                6 => {
                    self.reg[1] = self.adv(operand);
                }
                7 => {
                    self.reg[2] = self.adv(operand);
                }
                _ => unreachable!(),
            }
            self.ip += 2;
        }
        out
    }

    fn adv(&mut self, operand: u8) -> i128 {
        let num = self.a();
        let denom = 2i128.pow(self.combo(operand) as u32);
        num / denom
    }

    fn combo(&self, operand: u8) -> i128 {
        match operand {
            0..=3 => operand as i128,
            4 => self.a(),
            5 => self.b(),
            6 => self.c(),
            _ => unreachable!(),
        }
    }

    fn a(&self) -> i128 {
        self.reg[0]
    }
    fn b(&self) -> i128 {
        self.reg[1]
    }
    fn c(&self) -> i128 {
        self.reg[2]
    }
}

fn decoded_version(mut a: i64, output: &mut Vec<u8>, expected: Option<&[u8]>) {
    for i in 0.. {
        let mut b = (a ^ 1) % 8;
        b = (b ^ 5) ^ (a / (1 << b));
        let out = (b % 8) as u8;
        if let Some(expected) = expected {
            if i >= expected.len() || expected[i] != out {
                break;
            }
        }
        output.push(out);
        a = a >> 3;

        if a == 0 {
            break;
        }
    }
}

fn check_prefix(prefix: i64, size: usize, expected: &[u8]) -> bool {
    let mut output = vec![];
    for i in 0..8 {
        output.clear();
        let a = i << size | prefix;
        decoded_version(a, &mut output, Some(&expected));
        if !output.starts_with(expected) {
            return false;
        }
    }
    return true;
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let num: Vec<i128> = ints(input);

    let s = Instant::now();

    let prog: Vec<_> = num.iter().skip(3).map(|v| *v as u8).collect();

    let mut machine = Machine::new(num[0], num[1], num[2]);
    let result = machine.run(&prog, false);
    let part1 = result.iter().join(",");
    let part2 = find_solution_for_part2(&prog);

    let e = s.elapsed();

    if verify_expected {
        assert_eq!("7,5,4,3,4,5,3,4,6", part1);
        assert_eq!(164278899142333, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}

fn find_solution_for_part2(prog: &[u8]) -> i64 {
    /*
    After lots of printing and manual decoding of the program, it turns out
    that it consumes 'a' in 3 bits per loop iteration - at the end it shifts
    down by 3 bits. Looking at the outputs printed in 3bit sections, and
    only for input which have lots (6) first output bytes already correct,
    we can see that there are cycles of length 5 (actually, first one is len 1).
    Taking the smallest first values from those cycles and concatenating them
    forms a 21 bit prefix of the solution.
    */
    let mut result = vec![];
    decoded_version(38610541, &mut result, None);
    assert_eq!(vec![7u8, 5, 4, 3, 4, 5, 3, 4, 6], result);

    let max: i64 = 0b111_111_111_111_111_111_111_111_111_111_111;
    let size = max.ilog2() + 1;

    let prefixes: Vec<i64> = (0..max)
        .into_iter()
        .filter(|p| check_prefix(*p, size as usize, &prog[..6]))
        .take(10)
        .collect();

    let mut cycles: Vec<Vec<i64>> = vec![];
    cycles.push(nth_cycle(&prefixes, 0));
    cycles.push(nth_cycle(&prefixes, 1));
    cycles.push(nth_cycle(&prefixes, 2));
    cycles.push(nth_cycle(&prefixes, 3));
    cycles.push(nth_cycle(&prefixes, 4));
    cycles.push(nth_cycle(&prefixes, 5));
    cycles.push(nth_cycle(&prefixes, 6));

    let prefix = cycles[0][0]
        | cycles[1][0] << 3
        | cycles[2][0] << 6
        | cycles[3][0] << 9
        | cycles[4][0] << 12
        | cycles[5][0] << 15
        | cycles[6][0] << 18;

    let mut result = vec![];
    for v in 0.. {
        result.clear();
        let p = v << 21 | prefix;
        decoded_version(p, &mut result, Some(prog));
        if result == prog {
            return p;
        }
    }
    unreachable!()
}

#[test]
fn machine() {
    let mut m = Machine::new(0, 0, 9);
    m.run(&[2, 6], false);
    assert_eq!(1, m.b());

    let mut m = Machine::new(10, 0, 0);
    assert_eq!(vec![0, 1, 2], m.run(&[5, 0, 5, 1, 5, 4], false));

    let mut m = Machine::new(2024, 0, 0);
    assert_eq!(
        vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0],
        m.run(&[0, 1, 5, 4, 3, 0], false)
    );
    assert_eq!(0, m.a());

    let mut m = Machine::new(0, 29, 0);
    m.run(&[1, 7], false);
    assert_eq!(26, m.b());

    let mut m = Machine::new(0, 2024, 43690);
    m.run(&[4, 0], false);
    assert_eq!(44354, m.b());

    let mut m = Machine::new(729, 0, 0);
    assert_eq!(
        vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0],
        m.run(&[0, 1, 5, 4, 3, 0], false)
    );
    let mut m = Machine::new(117440, 0, 0);
    assert_eq!(vec![0, 3, 5, 4, 3, 0], m.run(&[0, 3, 5, 4, 3, 0], true));
}

fn nth_cycle(values: &[i64], n: usize) -> Vec<i64> {
    debug_assert_eq!(values.len() % 2, 0);
    let tmp: Vec<i64> = values.iter().map(|v| (v >> (n * 3)) & 0b111).collect();
    debug_assert_eq!(tmp[..values.len() / 2], tmp[values.len() / 2..]);
    tmp[..values.len() / 2].to_vec()
}
