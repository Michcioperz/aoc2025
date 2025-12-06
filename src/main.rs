use std::collections::{BTreeSet, HashSet};

use color_eyre::Result;
use itertools::Itertools;
use rayon::prelude::*;

macro_rules! time_is_a_force {
    ($e:expr) => {{
        let start = ::std::time::Instant::now();
        let res = $e;
        println!("{:?}", start.elapsed());
        res
    }};
}

fn main() -> Result<()> {
    color_eyre::install().unwrap();
    println!("{}", time_is_a_force!(task6b()?));
    Ok(())
}

fn task6b() -> Result<usize> {
    let input = include_str!("/Users/michcioperz/Downloads/6.input");
    Ok(input
        .lines()
        .fold(None, |acc: Option<Vec<Vec<char>>>, line| {
            if let Some(mut acc) = acc {
                for (v, c) in acc.iter_mut().zip(line.chars()) {
                    v.push(c);
                }
                Some(acc)
            } else {
                Some(line.chars().map(|c| vec![c]).collect_vec())
            }
        })
        .unwrap()
        .into_iter()
        .chain(vec![vec![]])
        .fold(
            (0usize, None),
            |(total, local): (usize, Option<(fn(usize, usize) -> usize, usize)>), mut col| match (
                local,
                col.iter().any(|c| !c.is_whitespace()),
            ) {
                (Some((op, elem)), true) => (
                    total,
                    Some((
                        op,
                        op(
                            elem,
                            col.into_iter()
                                .filter(|c| c.is_digit(10))
                                .collect::<String>()
                                .parse::<usize>()
                                .unwrap(),
                        ),
                    )),
                ),
                (None, true) => (
                    total,
                    Some((
                        if col.pop().unwrap() == '*' {
                            |a, b| a * b
                        } else {
                            |a, b| a + b
                        },
                        col.into_iter()
                            .filter(|c| c.is_digit(10))
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap(),
                    )),
                ),
                (Some((_, elem)), false) => (total + elem, None),
                (None, false) => (total, None),
            },
        )
        .0)
}

fn task5b() -> Result<usize> {
    let mut saw_empty = false;
    let input = if true {
        include_str!("/Users/michcioperz/Downloads/5.input")
    } else {
        "3-5
10-14
16-20
12-18

"
    };
    let (ids, ranges): (Vec<_>, _) = input.trim().lines().partition(|line| {
        if line.trim().is_empty() {
            saw_empty = true;
        }
        saw_empty
    });

    let mut fresh: Vec<(usize, usize)> = ranges
        .into_iter()
        .map(|line| line.trim().split_once('-').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .sorted()
        .collect_vec();

    dbg!(&fresh);

    fresh = fresh.into_iter().fold(vec![], |mut acc, x| {
        if acc.is_empty() {
            acc.push(x);
        } else {
            let y = acc.last_mut().unwrap();
            if y.1 >= x.0 - 1 {
                // cont = true;
                y.1 = y.1.max(x.1);
            } else {
                acc.push(x);
            }
        }
        acc
    });
    // dbg!(&fresh);

    Ok(fresh.into_iter().map(|r| r.1 - r.0 + 1).sum())
}

fn task5a() -> Result<usize> {
    let mut saw_empty = false;
    let (ids, ranges): (Vec<_>, _) = include_str!("/Users/michcioperz/Downloads/5.input")
        .trim()
        .lines()
        .partition(|line| {
            if line.trim().is_empty() {
                saw_empty = true;
            }
            saw_empty
        });

    let fresh: Vec<(usize, usize)> = ranges
        .into_iter()
        .map(|line| line.trim().split_once('-').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .sorted()
        .collect_vec();
    // .fold(vec![], |mut acc, x| {
    //     if acc.is_empty() {
    //         acc.push(x);
    //     } else {
    //         let y = acc.last_mut().unwrap();
    //         let (ya, yb) = *y;
    //         if x.0.max(y.1) <= x.1.max(y.1) {
    //             y.0 = ya.min(x.0);
    //             y.1 = yb.max(x.1);
    //         } else {
    //             acc.push(x);
    //         }
    //     }
    //     acc
    // });
    dbg!(&fresh);

    Ok(ids
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<usize>().unwrap())
        .filter(|&id| {
            fresh.iter().any(|r| r.0 <= id && r.1 >= id)
            // fresh
            //     .binary_search_by(|&(a, b)| {
            //         if id < a {
            //             std::cmp::Ordering::Greater
            //         } else if id > b {
            //             std::cmp::Ordering::Less
            //         } else {
            //             std::cmp::Ordering::Equal
            //         }
            //     })
            //     .is_ok()
        })
        .count())
}

fn task4b() -> Result<usize> {
    let eight_dirs = [-1isize, 0, 1]
        .into_iter()
        .cartesian_product([-1isize, 0, 1])
        .collect_vec();
    let mut map: HashSet<(isize, isize)> = include_str!("/Users/michcioperz/Downloads/4.input")
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '@').then_some((y as isize, x as isize)))
        })
        .collect();
    let mut total = 0usize;
    let mut cont = true;
    while cont {
        cont = false;
        let removals = map
            .iter()
            .copied()
            .filter(|&(y, x)| {
                eight_dirs
                    .iter()
                    .filter(|&(dy, dx)| map.contains(&(y + dy, x + dx)))
                    .count()
                    < 5
            })
            .collect_vec();
        cont = !removals.is_empty();
        total += removals.len();
        removals.iter().for_each(|yx| {
            map.remove(yx);
        });
    }
    Ok(total)
}

fn task4a() -> Result<usize> {
    let eight_dirs = [-1isize, 0, 1]
        .into_iter()
        .cartesian_product([-1isize, 0, 1])
        .collect_vec();
    let map: HashSet<(isize, isize)> = include_str!("/Users/michcioperz/Downloads/4.input")
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '@').then_some((y as isize, x as isize)))
        })
        .collect();
    Ok(map
        .iter()
        .copied()
        .filter(|&(y, x)| {
            eight_dirs
                .iter()
                .filter(|&(dy, dx)| map.contains(&(y + dy, x + dx)))
                .count()
                < 5
        })
        .count())
}

fn task3b() -> Result<usize> {
    Ok(include_str!("/Users/michcioperz/Downloads/3.input")
        .trim()
        .lines()
        // .par_bridge()
        .map(|l| {
            l.chars()
                .map(|c| usize::try_from(c.to_digit(10).unwrap()).unwrap())
                .collect_vec()
        })
        .map(|mut l| {
            let mut extras = l.split_off(l.len() - 11);
            extras.reverse();
            let mut answer = 0usize;
            loop {
                let (i, &digit) = l
                    .iter()
                    .enumerate()
                    .max_by_key(|(i, v)| (*v, usize::MAX - i))
                    .unwrap();
                answer = answer * 10 + digit;
                l.drain(0..=i);
                if extras.is_empty() {
                    break;
                } else {
                    l.push(extras.pop().unwrap());
                }
            }
            answer
        })
        .sum())
}
