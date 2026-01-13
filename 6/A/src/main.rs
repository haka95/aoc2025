use std::io;

fn main() {
    let mut nums: Vec<Vec<u64>> = Vec::new();
    let mut ops: Vec<String> = Vec::new();

    for line in io::stdin().lines() {
        if let Ok(line) = line {

            if line.contains("*") || line.contains("+") {
                ops = line.split_whitespace().filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
            } else {
                nums.push(line.split_whitespace().filter(|s| !s.is_empty()).map(|s| s.parse::<u64>().unwrap()).collect());
            }

        }
    }

    //In the following I strongly rely on that the input is correct, i.e. same number of numbers in each line and only + and * as operators
    let sum = nums.into_iter().fold(None, |acc, x| match acc {
        None => Some(x),
        Some(y) => {
            Some(
                y.iter().zip(x.iter()).enumerate().map(|(i, (a, b))|
                    if ops[i] == "+" { a + b } else { a * b }).collect())
        },
    }).map(|x| x.iter().sum()).unwrap_or(0);

    println!("{}", sum);
}
