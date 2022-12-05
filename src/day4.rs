use std::{path::PathBuf, ops::{RangeInclusive}, io::{BufReader, BufRead}, fs::File};




pub fn day4(path: &PathBuf) -> std::io::Result<()> {

    let lines = BufReader::new(File::open(path)?).lines();

    let mut sum_fully_contains: u32 = 0;
    let mut sum_overlaps: u32 = 0;

    for line in lines {
        match line {
            Ok(line) => {
                let vec = line
                    .trim()
                    .split(",")
                    .map(|s|s.trim())
                    .map(|s| {
                        let v = s.split("-")
                            .map(|s|s.trim())
                            .map(|s| s.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>();
                        v[0]..=v[1]
                    })
                    .collect::<Vec<RangeInclusive<u32>>>();

                sum_fully_contains += if fully_contains(&vec[0], &vec[1]) { 1 } else { 0 };
                sum_overlaps += if overlaps(&vec[0], &vec[1]) { 1 } else { 0 };
            },
            Err(_) => continue,
        }
    }

    println!("day4 part1 sum fully contains pairs: {}", sum_fully_contains);
    println!("day4 part2 sum overlaps pairs: {}", sum_overlaps);

    Ok(())
}


/// |------------|
///     |----|
fn fully_contains<Idx: PartialOrd>(range1: &RangeInclusive<Idx>, range2: &RangeInclusive<Idx>) -> bool {
    (range1.start() <= range2.start() && range1.end() >= range2.end())
        || (range2.start() <= range1.start() && range2.end() >= range1.end())
}

/// |------|
///    |------|
fn overlaps<Idx: PartialOrd>(range1: &RangeInclusive<Idx>, range2: &RangeInclusive<Idx>) -> bool {
    (range1.start() <= range2.start() && range1.end() >= range2.start())
        || (range2.start() <= range1.start() && range2.end() >= range1.start())
}
