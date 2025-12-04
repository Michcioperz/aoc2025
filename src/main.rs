use std::collections::HashSet;

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
    println!("{}", time_is_a_force!(task4b()?));
    Ok(())
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
