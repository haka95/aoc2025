use std::collections::HashMap;
use std::io;

fn main() {
    let mut sum = 0;

    //These are the three relevant datastructures which are first filled and then traversed:
    //Coordinates consist of line,col starting at 0 at the top left
    //Stores those coordinates which can be moved because they have less than 4 adjacent items
    let mut free: Vec<(u32, u32)> = Vec::new();
    //Stores which coordinates with an item are adjacent to an item at a specific coordinate
    let mut blocks: HashMap<(u32, u32), Vec<(u32, u32)>> = HashMap::new();
    //Stores the number of adjacent items for all items which are not yet in free or removed
    let mut blocked_by: HashMap<(u32, u32), u32> = HashMap::new();

    let mut pprev;
    let mut prev = None;
    let mut curr = None;
    let mut ln = 0;

    for line in io::stdin().lines() {
        if let Ok(line) = line {
            pprev = prev;
            prev = curr;
            curr = Some(line.chars().enumerate().flat_map(|(i, c)| (c == '@').then_some(i)).collect());

            if prev.is_some() {
                read_line(ln, &pprev, prev.as_ref().unwrap(), &curr, &mut free, &mut blocks, &mut blocked_by);
                ln += 1;
            }
        }
    }
    read_line(ln, &prev, curr.as_ref().expect("No line received!"), &None, &mut free, &mut blocks, &mut blocked_by);

    //println!("{:?}", blocks);
    while !free.is_empty() {
        let mut new_free: Vec<(u32, u32)> = Vec::new();
        sum += free.len();
        //println!("{:?}", free);

        for e in &free {
            if let Some(block_els) = blocks.get(e) {
                for e2 in block_els {
                    if blocked_by.contains_key(e2) {
                        let blocking_num = blocked_by.get(e2).unwrap() - 1;
                        if blocking_num < 4 {
                            new_free.push(*e2);
                            blocked_by.remove(e2);
                        } else {
                            blocked_by.entry(*e2).insert_entry(blocking_num);
                        }
                    }
                }
            }
        }

        free = new_free;
    }

    println!("{}", sum);
}

fn read_line(line: usize, prev : &Option<Vec<usize>>, curr : &Vec<usize>, next : &Option<Vec<usize>>,
             free: &mut Vec<(u32, u32)>, blocks: &mut HashMap<(u32, u32), Vec<(u32, u32)>>,
             blocked_by: &mut HashMap<(u32, u32), u32>) {



    let crawl_line = |pos: i32, l: &Vec<usize>| {
        l.iter().filter(|cp| **cp as i32 >= pos - 1 && **cp as i32 <= pos + 1).map(|e| *e).collect::<Vec<usize>>()
    };

    for pos in curr {
        let p = prev.as_ref().map(|l| crawl_line(*pos as i32, l)).unwrap_or_else(|| Vec::new());
        let mut c = crawl_line(*pos as i32, curr);
        let n = next.as_ref().map(|l| crawl_line(*pos as i32, l)).unwrap_or_else(|| Vec::new());

        c.retain(|e| e != pos);

        let mut positions = p.iter().map(|i| ((line - 1) as u32, *i as u32)).collect::<Vec<(u32, u32)>>();
        positions.extend(c.iter().map(|i| (line as u32, *i as u32)).collect::<Vec<(u32, u32)>>());
        positions.extend(n.iter().map(|i| ((line + 1) as u32, *i as u32)).collect::<Vec<(u32, u32)>>());

        let block_num = positions.len();
        blocks.insert((line as u32, *pos as u32), positions);

        if block_num < 4 {
            free.push((line as u32, *pos as u32));
        } else {
            blocked_by.insert((line as u32, *pos as u32), block_num as u32);
        }
    }
}
