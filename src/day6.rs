use std::{path::PathBuf, io::{BufReader, BufRead}, fs::File, collections::{VecDeque, HashMap, HashSet}};



pub fn day6(path: &PathBuf) -> std::io::Result<()> {
    let mut buf = String::new();
    BufReader::new(File::open(path)?)
        .read_line(&mut buf)
        .expect("read_line failed");

    let mut count = 0;
    let mut part1_count = None;
    let mut part2_count = None;

    const CAPACITY1: usize = 4;
    const CAPACITY2: usize = 14;

    let mut vec_deque1 = VecDeque::with_capacity(CAPACITY1);
    let mut vec_deque2 = VecDeque::with_capacity(CAPACITY2);

    for c in buf.chars() {
        count += 1;

        if vec_deque1.len() >= CAPACITY1 { vec_deque1.pop_front(); }
        if vec_deque2.len() >= CAPACITY2 { vec_deque2.pop_front(); }

        vec_deque1.push_back(c);
        vec_deque2.push_back(c);

        if part1_count.is_none() && vec_deque1.iter().collect::<HashSet<&char>>().len() == CAPACITY1 {
            part1_count = Some(count);
        }
        if part2_count.is_none() && vec_deque2.iter().collect::<HashSet<&char>>().len() == CAPACITY2 {
            part2_count = Some(count);
            break;
        }
    }

    println!("day6 part1 characters count: {}", part1_count.unwrap());
    println!("day6 part2 characters count: {}", part2_count.unwrap());

    Ok(())
}
