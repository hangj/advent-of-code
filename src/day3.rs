use std::{path::PathBuf, fs::File, io::{BufReader, BufRead, Lines}, collections::HashSet};



#[derive(Debug)]
struct Priority(u8);

/// 'a' -> 1
/// 'A' -> 27
impl From<char> for Priority {
    fn from(c: char) -> Self {
        Priority(
            if c.is_lowercase() {
                c as u8 - 'a' as u8 + 1
            } else {
                c as u8 - 'A' as u8 + 27
            })
    }
}

pub fn day3(path: &PathBuf) -> std::io::Result<()> {
    let lines = BufReader::new(
            File::open(path)?
        )
        .lines();

    let mut sum_priority : u32 = 0;
    for line in lines {
        match line {
            Ok(l) => {
                let s = l.trim();
                let compartment1 = &s[.. s.len()/2]
                    .chars()
                    .collect::<HashSet<char>>();
                let compartment2 = &s[s.len()/2 ..]
                    .chars()
                    .collect::<HashSet<char>>();

                let priority = compartment1
                    .intersection(compartment2)
                    .map(|c| Priority::from(*c))
                    .reduce(|acc, u| Priority(acc.0 + u.0) );

                sum_priority += if let Some(p) = priority {
                        p.0 as u32
                    } else {
                        0
                    }
            },
            Err(_) => continue,
        }
    }

    println!("day3 part1 sum_priority: {}", sum_priority);


    let mut iter = BufReader::new(
            File::open(path)?
        )
        .lines()
        .into_iter();


    let mut sum_priority: u32 = 0;

    loop {
        match get_group(&mut iter) {
            Some(lines) => {
                let priority = lines[0]
                    .chars()
                    .collect::<HashSet<char>>()
                    .intersection(&lines[1]
                        .chars()
                        .collect::<HashSet<char>>())
                    .map(|c| *c)
                    .collect::<HashSet<char>>()
                    .intersection(&lines[2]
                        .chars()
                        .collect::<HashSet<char>>())
                    .map(|c| Priority::from(*c))
                    .reduce(|acc, c| Priority(acc.0 + c.0) );
                match priority {
                    Some(p) => {
                        sum_priority += p.0 as u32;
                    },
                    None => continue,
                }
            },
            None => break,
        }
    }

    println!("day3 part2 sum_priority: {}", sum_priority);

    Ok(())
}

fn get_group(iter: &mut Lines<BufReader<File>>) -> Option<[String; 3]> {
    match iter.next() {
        Some(s) => Some([
            s.unwrap(),
            iter.next().unwrap().unwrap(),
            iter.next().unwrap().unwrap(),
        ]),
        None => None,
    }
    
}
