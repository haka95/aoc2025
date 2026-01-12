use std::io;

fn main() {
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in io::stdin().lines() {
        if let Ok(line) = line {

            if line == "" {
                break;
            }

            let mut split = line.split("-");
            let (f,t) = (split.next().unwrap().parse::<u64>().expect(format!("Unparsable number in split {line}").as_str()),
                         split.next().unwrap().parse::<u64>().expect(format!("Unparsable number in split {line}").as_str()));

            let len = ranges.len();
            let p1 = ranges.iter().position(|(_, int_t)| f <= *int_t).unwrap_or_else(|| len);
            let p2 = ranges.iter().rev().position(|(int_f, _)| t >= *int_f).map(|i| len - i - 1).unwrap_or_else(|| 0);
            //println!("{p1} {p2}");

            let (of, ot) = (ranges.get(p1).map(|e| e.0).unwrap_or_else(|| u64::MAX),
                            ranges.get(p2).map(|e| e.1).unwrap_or_else(|| u64::MIN));
            if p1 != p2 || t >= of {
                for i in (p1..=p2).rev() {
                    ranges.remove(i);
                }
            }
            let nf = if f < of {f} else {of};
            let nt = if t > ot || t < of {t} else {ot};
            ranges.insert(p1, (nf, nt));

            //println!("{:?}", ranges);
        }
    }

    let mut sum = 0;
    for (f,t) in &ranges {
        sum += t - f + 1;
    }

    println!("{}", sum);
}
