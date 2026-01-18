extern crate core;

use std::collections::HashMap;
use union_find::{UnionFind, UnionBySize, QuickUnionUf};
use std::io;

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
    let mut sizes = HashMap::new();

    for i in 0..dists.len() {
        //println!("{:?}", dists[i]);
        let e = dists[i];
        let c1 = uf.find(e.0.0);
        let c2 = uf.find(e.0.1);
        if c1 == c2 { continue; }
        uf.union(e.0.0, e.0.1);
        let nc = uf.find(e.0.1);
        let ncs: usize = sizes.get(&c1).copied().unwrap_or(1) + sizes.get(&c2).copied().unwrap_or(1);

        if ncs == coords.len() {
            println!("{}", coords[e.0.0].0 * coords[e.0.1].0);
            return;
        }

        sizes.entry(nc).and_modify(|x| *x = ncs).or_insert(ncs);

        //println!("{:?}", sizes);
    }
}
