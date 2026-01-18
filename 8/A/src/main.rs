extern crate core;

use std::collections::HashMap;
use union_find::{UnionFind, UnionBySize, QuickUnionUf};
use std::io;

const NO_UNIONS : usize = 1000;
const NO_MUL : usize = 3;

fn main() {
    let mut coords: Vec<(i64, i64, i64)> = Vec::new();
    let mut dists: Vec<((usize, usize), i64)> = Vec::new();

    for line in io::stdin().lines() {
        if let Ok(line) = line {
            let cs = line.split(",").collect::<Vec<&str>>().iter().map(|&x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            coords.push((cs[0], cs[1], cs[2]));

            if coords.len() >= 2 {
                for i in 0..coords.len() - 2 {
                    let (c1, c2) = (coords[i], coords.last().unwrap());
                    let (d1, d2, d3) = (c1.0 - c2.0, c1.1 - c2.1, c1.2 - c2.2);
                    let d = d1 * d1 + d2 * d2 + d3 * d3;

                    dists.push(((i, coords.len() - 1), d));
                }
            }
        }
    }

    dists.sort_by( |a, b| a.1.cmp(&b.1));

    let mut uf = QuickUnionUf::<UnionBySize>::new(coords.len());

    for i in 0..NO_UNIONS {
        //println!("{:?}", dists[i]);
        let e = dists[i];
        uf.union(e.0.0, e.0.1);
    }

    let mut sizes = HashMap::new();

    for i in 0..coords.len() {
        sizes.entry(uf.find(i)).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut max = sizes.values().collect::<Vec<&usize>>();
    max.sort_by(|a, b| b.cmp(a));
    max.truncate(NO_MUL);
    let prod : u64 = max.iter().map(|e| **e as u64).product();
    println!("{prod}");
}
