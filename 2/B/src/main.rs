#![feature(core_float_math)]
extern crate core;

use core::f32::math::powi;
use std::cmp::max;
use std::collections::HashSet;
use std::io;

fn main() {

    /*
     * Idea: For each interval iterate over repetitions and then try all
     * repeated combinations, e.g. for rep = 3: 111, 222, ..., 999, 101010...
     * It starts with the first few digits of the first interval boundary,
     * and ends when the second interval border is reached.
     * Duplicates are filtered out via memorizing in a HashSet and also
     * some other optimizations are contained.
     */

    if let Some(Ok(input)) = io::stdin().lines().next() {
        let mut sum = 0i64;

        for range in input.split(',') {
            let mut parts = range.split('-');
            let n1 = parts.next().and_then(|e| e.parse::<i64>().ok())
                .expect(format!("Failed to parse range: {}", range).as_str());
            let n2 = parts.next().and_then(|e| e.parse::<i64>().ok())
                .expect(format!("Failed to parse range: {}", range).as_str());

            let n1_digits = n1.ilog10() as i32 + 1;
            let n2_digits = n2.ilog10() as i32 + 1;
            let mut added = HashSet::<i64>::new();

            'l0: for rep in 2..=n2_digits {
                for i in 2..rep {
                    if rep % i == 0 {
                        continue 'l0;
                    }
                }

                let dig_min = max(1, n1_digits / rep);
                let start = powi(10f32, dig_min - 1) as i64;

                //println!("{rep} {start} {n2}");

                for part in start..=n2 {
                    let mut num = 0i64;
                    let dig = part.ilog10() as i32 + 1;

                    for k in 0..rep {
                        num += part * powi(10f32, k * dig) as i64;
                    }

                    if num > n2 {
                        continue 'l0;
                    }

                    if num >= n1 && !added.contains(&num) {
                        sum += num;
                        added.insert(num);
                        //println!("Added {num}");
                    }
                }
            }
        }

        println!("{}", sum);
    }
}
