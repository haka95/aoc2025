extern crate core;

use std::collections::HashMap;
use std::io;

fn main() {
    let mut pos: HashMap<i32, i64> = HashMap::new();

    for line in io::stdin().lines() {
        if let Ok(line) = line {

            if pos.is_empty() {
                pos = line.char_indices().filter(|(_, c)| *c == 'S').map(|(i, _)| (i as i32, 1)).collect();
            } else {
                let mut newpos: HashMap<i32, i64> = HashMap::new();
                for (i, n) in pos {
                    let np = if line.chars().nth(i as usize) == Some('^') {vec!((i - 1, n), (i + 1, n)) } else { vec!((i, n))};
                    let np = np.iter().filter(|(i, _)| *i >= 0 && *i < line.len() as i32 );
                    for (p, n) in np {
                        newpos.entry(*p).and_modify(|c| {*c += *n;}).or_insert(*n);
                    }
                }
                pos = newpos;
            }

            //println!("{:?}", pos);

        }
    }

    println!("{}",pos.values().sum::<i64>());
}
