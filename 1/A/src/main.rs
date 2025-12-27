use std::io::{self};
use std::process;

fn main() {
    let stdin = io::stdin();
    let mut pos = 50;
    let mut zeros = 0;

    for line in stdin.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                eprintln!("Could not read line");
                process::exit(1)
            },
        };

        if let Some((letter, number)) = parse_line(&line) {

            let snum = match letter {
                'L' => -number,
                'R' => number,
                _ => {
                    eprintln!("Received wrong leading letter: {letter}");
                    process::exit(1);
                }
            };

            pos += snum;
            pos %= 100; //Does not really calculate positions, as % may yield neg. results but does
                        //not matter for number of zeros.
            zeros += if pos == 0 {1} else {0};
        }
    }

    println!("{zeros}");
}

fn parse_line(s: &str) -> Option<(char, i32)> {
    let mut chars = s.chars();

    let letter = chars.next()?;
    let number: i32 = chars.collect::<String>().parse().ok()?;

    Some((letter, number))
}