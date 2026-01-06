use std::io;

fn main() {
    let mut sum = 0;
    let mut pprev;
    let mut prev = None;
    let mut curr = None;

    for line in io::stdin().lines() {
        if let Ok(line) = line {
            pprev = prev;
            prev = curr;
            curr = Some(line.chars().enumerate().flat_map(|(i, c)| (c == '@').then_some(i)).collect());

            if prev.is_some() {
                sum += check_valid_pos(&pprev, prev.as_ref().unwrap(), &curr);
            }
        }
    }
    sum += check_valid_pos(&prev, curr.as_ref().expect("No line received!"), &None);

    println!("{}", sum);
}

fn check_valid_pos(prev : &Option<Vec<usize>>, curr : &Vec<usize>, next : &Option<Vec<usize>>) -> usize {
    let lc = |pos: i32, l: &Option<Vec<usize>>| {
        l.as_ref().map(|l| l.iter().filter(|cp| **cp as i32 >= pos - 1 && **cp as i32 <= pos + 1).count()).unwrap_or(0)
    };

    curr.iter().filter(
        |pos| {
            let cntp = lc(**pos as i32, prev);
            let cntc = curr.iter().filter(|cp| **cp as i32 == **pos as i32 - 1 || **cp == **pos + 1).count();
            let cntn = lc(**pos as i32, next);

            (cntp + cntc + cntn) < 4
        }).count()
}
