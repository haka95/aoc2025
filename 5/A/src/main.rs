use std::io;

fn main() {
    let mut sum = 0;
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut first_input_sec = true;

    for line in io::stdin().lines() {
        if let Ok(line) = line {
            if line == "" {
                first_input_sec = false;
                ranges.sort_by(|a, b| b.1.cmp(&a.1));
                continue;
            }
            if first_input_sec {
                let mut split = line.split("-");
                let (f,t) = (split.next().unwrap().parse::<u64>().expect(format!("Unparsable number in split {line}").as_str()),
                             split.next().unwrap().parse::<u64>().expect(format!("Unparsable number in split {line}").as_str()));
                ranges.push((f,t));
            } else {
                let num = line.parse::<u64>().expect(format!("Unparsable number in input {line}").as_str());
                for (f,t) in &ranges {
                    if num >= *f && num <= *t {
                        sum += 1;
                        break;
                    } else if num > *t {
                        break;
                    }
                }
            }
        }
    }

    println!("{}", sum);
}
