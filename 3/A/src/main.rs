use std::io;

fn main() {
    let mut sum = 0;

    for line in io::stdin().lines() {
        if let Ok(line) = line {
            let mut h1 = None;
            let mut h2 = None;
            let mut ci = None;
            let mut last_ci;

            for c in line.chars() {
                last_ci = ci;
                ci = Some(c.to_digit(10).expect("invalid digit {c}"));

                if let Some(lci) = last_ci {
                    if h1.is_none() || h1.unwrap() < lci {
                        h1 = last_ci;
                        h2 = ci;
                    } else if h2.unwrap() < ci.unwrap() {
                        h2 = ci;
                    }
                }
            }

            let num = 10 * h1.expect("Not enough digits received") +
                h2.expect("Not enough digits received");

            sum += num;
            //println!("{}", num);
        }
    }

    println!("{}", sum);
}
