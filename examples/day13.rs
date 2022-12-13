use std::{cmp::Ordering, convert::Infallible, str::FromStr};

use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day13.txt");

#[derive(Clone, Debug, PartialEq, Eq)]
enum Data {
    List(Vec<Data>),
    Int(u32),
}

fn cmp_list(a: &[Data], b: &[Data]) -> Option<std::cmp::Ordering> {
    for (a, b) in a.iter().zip(b.iter()) {
        let cmp = a.partial_cmp(b);
        if let Some(cmp) = cmp {
            if cmp != std::cmp::Ordering::Equal {
                return Some(cmp);
            }
        }
    }

    let len_cmp = a.len().cmp(&b.len());
    if len_cmp != std::cmp::Ordering::Equal {
        return Some(len_cmp);
    }
    None
}

impl std::cmp::PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Data::Int(a), Data::Int(b)) => Some(a.cmp(b)),
            (Data::List(a), Data::List(b)) => cmp_list(a, b),
            (Data::List(va), Data::Int(b)) => {
                let vb = vec![Data::Int(*b)];
                cmp_list(va, &vb)
            }
            (Data::Int(a), Data::List(vb)) => {
                let va = vec![Data::Int(*a)];
                cmp_list(&va, vb)
            }
        }
    }
}

impl std::str::FromStr for Data {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Data, Self::Err> {
        // Input like "[1,[2,[3,[4,[5,6,7]]]],8,9]"

        struct Parser {
            depth: usize,
            head: Vec<Data>,
        }

        impl Parser {
            fn tokenize(s: &str) -> Vec<String> {
                let mut ret = vec![];
                let mut cur = String::new();
                for c in s.chars() {
                    if ['[', ']', ','].contains(&c) {
                        if !cur.is_empty() {
                            ret.push(cur.clone());
                            cur.clear();
                        }
                        ret.push(c.to_string());
                    } else {
                        cur.push(c);
                    }
                }
                ret
            }

            fn get_tail_mut(&mut self) -> &mut Vec<Data> {
                let mut tail = &mut self.head;
                for _ in 0..self.depth {
                    tail = match tail.last_mut().unwrap() {
                        Data::List(l) => l,
                        Data::Int(_) => panic!("cannot push list into int"),
                    };
                }
                tail
            }
            fn push_list(&mut self, list: Vec<Data>) {
                self.get_tail_mut().push(Data::List(list));
                self.depth += 1;
            }
            fn pop_list(&mut self) {
                self.depth -= 1;
            }
            fn push_int(&mut self, i: u32) {
                self.get_tail_mut().push(Data::Int(i));
            }

            fn parse(s: &str) -> Data {
                let mut p = Parser {
                    depth: 0,
                    head: vec![],
                };

                let toks = Self::tokenize(s);
                for tok in toks {
                    match tok.as_str() {
                        "[" => {
                            p.push_list(vec![]);
                        }
                        "]" => {
                            p.pop_list();
                        }
                        "," => {}
                        numchar => {
                            p.push_int(numchar.parse::<u32>().unwrap());
                        }
                    }
                }
                let x = p.head.drain(..).next().unwrap();
                x
            }
        }

        Ok(Parser::parse(s))
    }
}

fn main() -> anyhow::Result<()> {
    let pairs: Vec<(Data, Data)> = INPUT
        .split("\n\n")
        .map(|pair| {
            let (first, second) = pair.split_once("\n").expect("malformed input");
            (
                Data::from_str(first).unwrap(),
                Data::from_str(second).unwrap(),
            )
        })
        .collect_vec();

    let indices = pairs.iter().enumerate().filter_map(|(idx, (a, b))| {
        if match a.partial_cmp(b) {
            Some(ord) => match ord {
                Ordering::Less => true,
                Ordering::Equal => true,
                Ordering::Greater => false,
            },
            None => false,
        } {
            Some(idx + 1)
        } else {
            None
        }
    });

    println!("Part 1: {}", indices.sum::<usize>());
    println!("Part 2: {}", 0);
    Ok(())
}
