extern crate core;

use std::collections::HashSet;
use std::io;

fn main() {
    let mut pos: HashSet<i32> = HashSet::new();
    let mut sum = 0;

    for line in io::stdin().lines() {
        if let Ok(line) = line {

            if pos.is_empty() {
                pos = line.char_indices().filter(|(_, c)| *c == 'S').map(|(i, _)| i as i32).collect();

            } else {
                pos = pos.iter().flat_map(|i| if line.chars().nth(*i as usize) == Some('^') { sum += 1; vec!(i- 1, i+1) } else { vec!(*i)}).filter(|i| *i >= 0 && *i < line.len() as i32 ).collect();
            }

            //println!("{:?}", pos);

        }
    }

    println!("{}",sum);
}
