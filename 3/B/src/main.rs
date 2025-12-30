use std::io;

fn main() {
    let mut sum = 0;

    for line in io::stdin().lines() {
        if let Ok(line) = line {
            let mut res = Vec::<u32>::new();

            for c in line.chars().rev() {
                let ci = c.to_digit(10).expect("invalid digit {c}");
                if res.len() < 12 {
                    res.push(ci);
                } else if *res.get(11).unwrap() <= ci {
                    res.push(ci);
                    let mut del = 0;
                    for i in (1..=11).rev() {
                        if res.get(i).unwrap() < res.get(i-1).unwrap() {
                            del = i;
                            break;
                        }
                    }
                    res.remove(del);
                }
            }

            let mut num = 0u64;
            for d in res.iter().rev() {
                num = 10 * num + (*d as u64);
            }

            //println!("{}", num);
            sum += num;
        }
    }

    println!("{}", sum);
}
