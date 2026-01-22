extern crate core;

use std::io;

fn main() {

    let mut coords: Vec<(i64, i64)> = Vec::new();
    let mut a_max = 0i64;

    for line in io::stdin().lines() {
        if let Ok(line) = line {
            let (x, y) = line.split_once(',').map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())).unwrap();

            for (cx, cy) in coords.iter() {
                let d1 = cx - x + 1;
                let d2 = cy - y + 1;

                let a = d1.abs() * d2.abs();

                a_max = if a_max < a { a } else { a_max };
            }

            coords.push((x, y));
        }
    }

    println!("{}", a_max);
}
