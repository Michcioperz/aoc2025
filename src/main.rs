use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
};

use color_eyre::Result;
use indicatif::ProgressIterator;
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

macro_rules! edbg {
    ($e:expr) => {{
        let res = $e;
        eprintln!("{:?}", &res);
        res
    }};
}

fn main() -> Result<()> {
    color_eyre::install().unwrap();
    println!("{}", time_is_a_force!(task10b()?));
    Ok(())
}

fn task10b() -> Result<usize> {
    let input = if true {
        include_str!("/Users/michcioperz/Downloads/10.input")
    } else {
        todo!()
    };
    Ok(input
        .lines()
        .map(|line| line.split_once(' ').unwrap().1.rsplit_once(' ').unwrap())
        .map(|(toggles, goal)| {
            (
                goal.trim_matches(['{', '}'])
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_vec(),
                toggles
                    .split_whitespace()
                    .map(|toggle| {
                        toggle
                            .trim_matches(['(', ')'])
                            .split(',')
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect_vec()
                    })
                    .collect_vec(),
            )
        })
        .map(|(goal, toggles)| {
            let mut states = HashSet::new();
            states.insert(vec![0; goal.len()]);
            let mut steps = 0;
            while !states.contains(&goal) {
                steps += 1;
                dbg!(steps, states.len());
                states = states
                    .into_iter()
                    .flat_map(|ini| {
                        toggles.iter().map(move |toggle| {
                            let mut new = ini.clone();
                            for &t in toggle {
                                *new.get_mut(t).unwrap() += 1;
                            }
                            new
                        })
                    })
                    .filter(|state| state.iter().zip(goal.iter()).all(|(got, want)| got <= want))
                    .collect();
            }
            steps
        })
        .progress_count(172)
        .sum())
}

fn task10a() -> Result<usize> {
    let input = if true {
        include_str!("/Users/michcioperz/Downloads/10.input")
    } else {
        todo!()
    };
    Ok(input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(goal, toggles)| {
            (
                goal.len() - 2,
                goal.trim_matches(['[', ']'])
                    .chars()
                    .map(|c| c == '#')
                    .enumerate()
                    .map(|(i, b)| usize::from(b) << i)
                    .sum(),
                toggles
                    .split_whitespace()
                    .take_while(|x| x.starts_with('('))
                    .map(|toggle| {
                        toggle
                            .trim_matches(['(', ')'])
                            .split(',')
                            .map(|x| 1 << x.parse::<usize>().unwrap())
                            .sum()
                    })
                    .collect_vec(),
            )
        })
        .map(|(n, goal, toggles)| {
            let mask = (1 << n) - 1;
            let mut states = HashSet::new();
            states.insert(0);
            let mut steps = 0;
            while !states.contains(&goal) {
                steps += 1;
                states = states
                    .into_iter()
                    .flat_map(|ini| toggles.iter().map(move |toggle| (ini ^ toggle) & mask))
                    .collect();
            }
            steps
        })
        .sum())
}

fn task9b() -> Result<usize> {
    let input = if true {
        include_str!("/Users/michcioperz/Downloads/9.input")
    } else {
        r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#
    };
    let reds = input
        .trim()
        .lines()
        .map(|x| {
            x.split_once(',')
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .unwrap()
        })
        .collect_vec();
    let mut good_tiles: HashSet<_> = reds.iter().cloned().collect();
    for green in reds
        .iter()
        .cloned()
        .circular_tuple_windows()
        .flat_map(|(x, y)| {
            (x.0.min(y.0)..=x.0.max(y.0)).cartesian_product(x.1.min(y.1)..=x.1.max(y.1))
        })
    {
        good_tiles.insert(green);
    }
    fn on_off_vector(sorted_greens: impl Iterator<Item = usize>) -> Vec<(usize, bool)> {
        let mut last_good = None;
        let mut res = sorted_greens.fold(vec![], |mut acc, z| {
            last_good = Some(z);
            match acc.last() {
                None => {
                    acc.push((z, true));
                }
                Some(&(q, false)) => {
                    if q == z {
                        acc.pop();
                    } else {
                        acc.push((z, true));
                    }
                }
                Some(&(_, true)) => {
                    acc.push((z + 1, false));
                }
            }
            acc
        });
        if res.last().unwrap().1 {
            res.push((last_good.unwrap() + 1, false));
        }
        res
    }
    let rows: HashMap<_, _> = good_tiles
        .iter()
        .cloned()
        .into_group_map_by(|z| z.0)
        .into_iter()
        .map(|(k, mut v)| {
            v.sort();
            (k, on_off_vector(v.into_iter().map(|z| z.1)))
        })
        .collect();
    let cols: HashMap<_, _> = good_tiles
        .iter()
        .cloned()
        .into_group_map_by(|z| z.1)
        .into_iter()
        .map(|(k, mut v)| {
            v.sort();
            (k, on_off_vector(v.into_iter().map(|z| z.0)))
        })
        .collect();
    Ok(reds
        .iter()
        .tuple_combinations()
        .filter(|&(x, y)| {
            let xmin = x.0.min(y.0);
            let xmax = x.0.max(y.0);
            let ymin = x.1.min(y.1);
            let ymax = x.1.max(y.1);
            fn subsolve(row: Option<&Vec<(usize, bool)>>, a: usize, b: usize) -> bool {
                let Some(row) = row else { return false };
                for &(q, on) in row {
                    match (q.cmp(&a), q.cmp(&b), on) {
                        (Ordering::Less, _, _) => continue,
                        (Ordering::Equal, _, false) => return false,
                        (Ordering::Equal, _, true) => continue,
                        (Ordering::Greater, Ordering::Less | Ordering::Equal, _) => return false,
                        (Ordering::Greater, Ordering::Greater, false) => return true,
                        (Ordering::Greater, Ordering::Greater, true) => return false,
                    }
                }
                false
            }
            ([
                (rows.get(&xmin), ymin, ymax),
                (rows.get(&xmax), ymin, ymax),
                (cols.get(&ymin), xmin, xmax),
                (cols.get(&ymax), xmin, xmax),
            ])
            .into_iter()
            .all(|(r, a, b)| subsolve(r, a, b))
            // (ymin..=ymax)
            //     .flat_map(|y| [(xmin, y), (xmax, y)])
            //     .chain((xmin..=xmax).flat_map(|x| [(x, ymin), (x, ymax)]))
            //     .all(|z| good_tiles.contains(&z))
        })
        .map(|(x, y)| x.0.abs_diff(y.0).saturating_add(1) * x.1.abs_diff(y.1).saturating_add(1))
        .max()
        .unwrap())
}

fn task9a() -> Result<usize> {
    let input = if true {
        include_str!("/Users/michcioperz/Downloads/9.input")
    } else {
        r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#
    };
    Ok(input
        .trim()
        .lines()
        .map(|x| {
            x.split_once(',')
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .unwrap()
        })
        .tuple_combinations()
        .map(|(x, y)| x.0.abs_diff(y.0).saturating_add(1) * x.1.abs_diff(y.1).saturating_add(1))
        .max()
        .unwrap())
}

fn task8b() -> Result<usize> {
    let input: Vec<(usize, usize, usize)> = include_str!("/Users/michcioperz/Downloads/8.input")
        .lines()
        .map(|it| {
            it.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    fn dist(x: (usize, usize, usize), y: (usize, usize, usize)) -> usize {
        x.0.abs_diff(y.0).pow(2) + x.1.abs_diff(y.1).pow(2) + x.2.abs_diff(y.2).pow(2)
    }
    let mut cords: HashMap<usize, HashSet<usize>> = HashMap::new();
    let (a, b) = (0..input.len())
        .tuple_combinations::<(_, _)>()
        .sorted_by_key(|&(a, b)| dist(input[a], input[b]))
        .inspect({
            let mut i = 0;
            move |v| {
                eprintln!("{i} {v:?}");
                i += 1;
            }
        })
        .take_while_inclusive(|&(a, b)| {
            cords.entry(a).or_default().insert(b);
            cords.entry(b).or_default().insert(a);
            let mut q = vec![0];
            let mut count = 0;
            let mut visited = vec![false; input.len()];
            while let Some(j) = q.pop() {
                if !visited[j] {
                    if let Some(conns) = cords.get(&j) {
                        for &conn in conns {
                            if !visited[conn] {
                                q.push(conn);
                            }
                        }
                    }
                    visited[j] = true;
                    count += 1;
                }
            }
            count != input.len()
        })
        .last()
        .unwrap();
    Ok(input[a].0 * input[b].0)
}

fn task8a() -> Result<usize> {
    let input: Vec<(usize, usize, usize)> = include_str!("/Users/michcioperz/Downloads/8.input")
        .lines()
        .map(|it| {
            it.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    fn dist(x: (usize, usize, usize), y: (usize, usize, usize)) -> usize {
        x.0.abs_diff(y.0).pow(2) + x.1.abs_diff(y.1).pow(2) + x.2.abs_diff(y.2).pow(2)
    }
    let cords: HashMap<usize, HashSet<usize>> = (0..input.len())
        .tuple_combinations::<(_, _)>()
        .sorted_by_key(|&(a, b)| dist(input[a], input[b]))
        .take(1000)
        .flat_map(|x| [x, (x.1, x.0)])
        .into_grouping_map()
        .collect();
    let mut visited = vec![false; input.len()];
    Ok((0..input.len())
        .filter_map(|i| {
            if visited[i] {
                return None;
            }
            let mut q = vec![i];
            let mut count = 0;
            while let Some(j) = q.pop() {
                if !visited[j] {
                    if let Some(conns) = cords.get(&j) {
                        for &conn in conns {
                            if !visited[conn] {
                                q.push(conn);
                            }
                        }
                    }
                    visited[j] = true;
                    count += 1;
                }
            }
            Some(count)
        })
        .sorted()
        .rev()
        .take(3)
        .product())
}

fn task7b() -> Result<usize> {
    let mut input = include_str!("/Users/michcioperz/Downloads/7.input")
        .lines()
        .map(|it| it.chars().collect_vec());
    let mut beams: HashMap<usize, usize> = vec![(
        input
            .next()
            .unwrap()
            .into_iter()
            .find_position(|&c| c == 'S')
            .unwrap()
            .0,
        1,
    )]
    .into_iter()
    .collect();
    let mut map = input.collect_vec();
    for line in &mut map {
        beams = beams
            .into_iter()
            .flat_map(|(beam, count)| {
                if line[beam] == '^' {
                    vec![(beam.wrapping_sub(1), count), (beam.wrapping_add(1), count)]
                } else {
                    vec![(beam, count)]
                }
            })
            .filter(|(x, _)| (0..line.len()).contains(x))
            .into_grouping_map()
            .sum();
    }
    Ok(beams.into_values().sum())
}

fn task7a() -> Result<usize> {
    let mut input = include_str!("/Users/michcioperz/Downloads/7.input")
        .lines()
        .map(|it| it.chars().collect_vec());
    let mut beams = vec![
        input
            .next()
            .unwrap()
            .into_iter()
            .find_position(|&c| c == 'S')
            .unwrap()
            .0,
    ];
    let mut map = input.collect_vec();
    let mut splits = 0usize;
    for line in &mut map {
        beams = beams
            .into_iter()
            .flat_map(|beam| {
                if line[beam] == '^' {
                    splits += 1;
                    vec![beam.wrapping_sub(1), beam.wrapping_add(1)]
                } else {
                    vec![beam]
                }
            })
            .filter(|x| (0..line.len()).contains(x))
            .dedup()
            .collect_vec();
    }
    Ok(splits)
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
