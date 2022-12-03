use std::fs::File;
use std::io::{BufReader, BufRead, Lines};
use std::path::PathBuf;



pub fn day1(path: PathBuf) -> std::io::Result<()> {
    let mut iter = BufReader::new(
            File::open(path)?
        )
        .lines()
        .into_iter();


    let mut max = 0u64;
    let mut top3 = [0u64,0,0];
    loop {
        match get_one(&mut iter) {
            Some(n) => {
                if n > max { max = n; }
                for i in 0..3 {
                    if n > top3[i] {
                        top3[i] = n;
                        top3.sort();
                        break;
                    }
                }
            },
            None => break,
        }
    }

    println!("day1 part1 max: {}", max);
    // println!("top3: {:#?}", top3);
    println!("day1 part2 sum of top3: {}", top3[0] + top3[1] + top3[2]);

    Ok(())
}

fn get_one(iter: &mut Lines<BufReader<File>>) -> Option<u64> {
    let mut sum = 0u64;
    loop {
        match iter.next() {
            Some(Ok(mut s)) => {
                s = s.trim().to_string();
                if s.is_empty() {
                    break;
                } else {
                    sum += s.parse::<u64>().unwrap();
                }
            },
            _ => return None,
        }

    }

    Some(sum)
}
