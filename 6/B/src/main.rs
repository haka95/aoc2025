extern crate core;
use std::io;

fn main() {
    let mut lines:Vec<Vec<char>> = Vec::new();
    let mut ops: String = "".to_string();

    for line in io::stdin().lines() {
        if let Ok(line) = line {

            if line.contains("*") || line.contains("+") {
                ops = line;
            } else {
                lines.push(line.chars().collect());
            }

        }
    }

    let mut sum = 0;
    let mut curr_op = ' ';
    let mut cs = 0;
    let mut ne = 0;

    for (i,c) in ops.char_indices() {
        let mut st = "".to_string();
        for s in lines.iter() {
            if s[i] != ' ' {
                st.push(s[i]);
            }
        }
        let num = st.parse::<u64>().unwrap_or(ne);
        //println!("{num}");

        if c != ' ' {
            curr_op = c;
            sum += cs;
            ne = if curr_op == '+' { 0 } else { 1 };
            cs = ne;
        }

        if curr_op == '+' {
            cs += num;
        } else {
            cs *= num;
        }
    }
    sum += cs;

    println!("{}", sum);
}
