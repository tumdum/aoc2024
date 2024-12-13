use crate::input::token_groups;
use anyhow::Result;
use std::time::{Duration, Instant};

type Pos = crate::pos::Pos<i64>;

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<Vec<String>> = token_groups(input, "\n", None);

    let s = Instant::now();

    let mut input = vec![];
    for config in lines.chunks_exact(3) {
        let (a, b, prize) = (&config[0], &config[1], &config[2]);
        let (ax, ay) = (int(&a[2]), int(&a[3]));
        let (bx, by) = (int(&b[2]), int(&b[3]));
        let (px, py) = (int(&prize[1]), int(&prize[2]));
        assert!(ax >= 0 && ay >= 0 && bx >= 0 && by >= 0 && px >= 0 && py >= 0);
        let (a, b, p) = (Pos::new(ax, ay), Pos::new(bx, by), Pos::new(px, py));
        input.push((a, b, p));
    }

    let part1: u64 = input
        .iter()
        .flat_map(|(a, b, p)| solve_equations(*a, *b, *p))
        .sum();

    let part2: u64 = input
        .into_iter()
        .flat_map(|(a, b, p)| {
            solve_equations(a, b, Pos::new(p.x + 10000000000000, p.y + 10000000000000))
        })
        .sum();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(39290, part1);
        assert_eq!(73458657399094, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}

fn solve_equations(a: Pos, b: Pos, p: Pos) -> Option<u64> {
    let alpha = |px: i64, bx: i64, beta: i64, ax: i64| -> i64 { (px - bx * beta) / ax };

    let (ax, ay, bx, by, px, py) = (a.x, a.y, b.x, b.y, p.x, p.y);
    let checkx = |alpha: i64, beta: i64| ax * alpha + bx * beta == px;
    let checky = |alpha: i64, beta: i64| ay * alpha + by * beta == py;
    let check = |alpha, beta| checkx(alpha, beta) && checky(alpha, beta);

    // 1. ax * alpha + bx * beta = px
    // 2. ay * alpha + by * beta = py

    // 1. ax * alpha             = px - bx * beta
    // 2. ay * alpha + by * beta = py

    // 1.      alpha             = (px - bx * beta) / ax
    // 2. ay * alpha + by * beta = py

    // 2. ay *            ((px - bx * beta) / ax) +      by * beta = py
    //    ay *            ((px - bx * beta)     ) + ax * by * beta = py * ax
    //    ay * px         - ay * bx * beta        + ax * by * beta = py * ax
    //                    - ay * bx * beta        + ax * by * beta = py * ax - ay * px
    //    ax * by * beta  - ay * bx * beta                         = py * ax - ay * px
    //    beta * (ax * by - ay * bx )                              = py * ax - ay * px
    //    beta                                                     = (py * ax - ay * px) / (ax * by - ay * bx )
    let beta = |px: i64, py: i64, ax: i64, ay: i64, bx: i64, by: i64| -> i64 {
        (py * ax - ay * px) / (ax * by - ay * bx)
    };

    let beta = beta(p.x, p.y, a.x, a.y, b.x, b.y);
    let alpha = alpha(px, bx, beta, ax);
    if check(alpha, beta) {
        Some(alpha as u64 * 3 + beta as u64)
    } else {
        None
    }
}

fn int(s: impl AsRef<str>) -> i64 {
    let s: String = s.as_ref().chars().filter(|c| c.is_ascii_digit()).collect();
    s.parse()
        .expect(&format!("Failed to parse '{}'", s.as_str()))
}
