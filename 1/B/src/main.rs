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

            if pos < 0 || pos == 0 && letter == 'L' {
                pos += 100;
            } else if pos > 100 || pos == 100 && letter == 'R'{
                pos -= 100;
            }

            let extra_turns = snum.abs() / 100;
            let remainder = snum % 100;
            pos += remainder;
            zeros += extra_turns + if pos <= 0 || pos >= 100 { 1 } else { 0 };
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