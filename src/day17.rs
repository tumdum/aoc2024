use anyhow::Result;
use itertools::Itertools;
use std::time::{Duration, Instant};

use crate::input::{ints, tokens};

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
        // println!("{out:?}");
        out
    }

    fn adv(&mut self, operand: u8) -> i128 {
        let num = self.a();
        let denom = 2i128.pow(self.combo(operand) as u32);
        // TODO: trunc?
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

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let num: Vec<i128> = ints(input);

    dbg!(&num);

    let mut machine = Machine::new(num[0], num[1], num[2]);
    let result = machine.run(&num[3..].iter().map(|v| *v as u8).collect_vec(), false);
    println!("{}", result.iter().join(","));

    println!("part2");
    for i in 0.. {
        let mut machine = Machine::new(i, num[1], num[2]);
        let prog = num[3..].iter().map(|v| *v as u8).collect_vec();
        let result = machine.run(&prog, true);
        if result == prog {
            println!("result {}", result.iter().join(","));
            println!("found a: {i}");
            break;
        }
        if i % 10000000 == 0 {
            println!("a: {i}");
        }
    }

    let s = Instant::now();

    let e = s.elapsed();

    if verify_expected {
        // assert_eq!(0, part1);
        // assert_eq!(0, part2);
    }
    if output {
        // println!("\t{}", part1);
        // println!("\t{}", part2);
    }
    Ok(e)
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
