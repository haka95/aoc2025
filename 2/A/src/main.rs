#![feature(core_float_math)]
extern crate core;

use core::f32::math::powi;
use std::io;

fn main() {

    /*
     * Non-streight-forward solution trying to solve the problem without looping over all possible numbers
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


            /*
             * The following blocks determine
             * - The lowest/highest number which (repeated twice) is between n1 and n2
             * - The number of digits this number has
             *
             * The idea is that all number between n1_front and n2_front can be iterated and
             * concatenated with itself to get the numbers to be summed up.
             */
            let (n1_front, n1_m_digits) = if n1_digits % 2 == 0 {
                let dig = n1_digits / 2;
                let fact = powi(10f32, dig) as i64;
                let mut num = n1 / fact;
                if num * fact + num < n1 {
                    num += 1;
                }
                (num, dig)
            } else {
                let dig = n1_digits / 2 + 1;
                (powi(10f32, dig - 1) as i64, dig)
            };

            let (n2_front, n2_m_digits) = if n2_digits % 2 == 0 {
                let mut dig = n2_digits / 2;
                let fact = powi(10f32, dig) as i64;
                let mut num = n2 / fact;
                if num * fact + num > n2 {
                    num -= 1;
                    dig = num.ilog10() as i32 + 1
                }
                (num, dig)
            } else {
                let dig = n2_digits / 2;
                (powi(10f32, dig) as i64 - 1, dig)
            };


            //We skip if no numbers are within the range, e.g because the second number in the
            //range is lower than the first one.
            if n1_front > n2_front && n1_m_digits > n2_m_digits {
                continue;
            }

            //let from = n1_front + n1_front * (powi(10f32, n1_m_digits) as i32);
            //let to = n2_front + n2_front * (powi(10f32, n2_m_digits) as i32);
            //println!("{} - {} // {} {} // {} {} ---> {} - {}", n1, n2, n1_front, n1_m_digits, n2_front, n2_m_digits, from, to);

            //To sum up all the numbers we use sum_per_digit which uses a variation of Gaussian formula to
            //sum all the self concatenated numbers from f to t.
            //sum_per_digit however only works for a fixed digit number, so we iterate over possible digits.
            for d in n1_m_digits..=n2_m_digits {
                let f = if d == n1_m_digits {Some(n1_front)} else {None};
                let t = if d == n2_m_digits {Some(n2_front)} else {None};

                sum += sum_per_digit(d, f, t);
                //println!("{} - {}: {}", f, t, sum_per_digit(d, f, t));
            }
        }

        println!("{}", sum);
    }
}

/// Computes the arithmetic sum of all `d`-digit numbers concatenated with
/// itself starting with start, ending with stop, if Some, otherwise it
/// starts with the lowest number of `d` digits and stops with the highest.
///
/// I.e.
///
/// `sum_per_digit(3, None, None)` computes 100100 + 101101 + 102102 + ... + 998998 + 999999
/// `sum_per_digit(2, Some(13), None)` computes 1313 + 1414 + 1515 + ... + 9999
/// `sum_per_digit(3, Some(200), Some(350))` computes 200200 + 201201 + ... + 349349 + 350350
///
fn sum_per_digit(d: i32, start: Option<i64>, stop: Option<i64>) -> i64 {
    let f = start.unwrap_or(powi(10f32, d-1) as i64);
    let t = stop.unwrap_or(powi(10f32, d) as i64 - 1);

    ((f + t) * (t - f + 1)) / 2i64 * (1 + (powi(10f32, d) as i64))
}
