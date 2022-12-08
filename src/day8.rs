use std::{path::PathBuf, io::{BufReader, BufRead}, fs::File, collections::HashSet};




pub fn day8(path: &PathBuf) -> std::io::Result<()> {
    let lines = BufReader::new(File::open(path)?).lines();
    let mut forest = Vec::new();
    for line in lines {
        match line {
            Ok(line) => {
                let row = line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>();

                forest.push(row);
            },
            Err(_) => break,
        }
    }

    println!("forest: {:?}", forest);

    // how many trees are visible from outside the grid?
    let mut all_visible = HashSet::new();

    let mut left2rght = HashSet::new();
    
    for j in 0..forest.len() {
        let row = &forest[j];

        // from left to right
        let mut last = None;
        for i in 0..row.len() {
            let t = row[i];
            match last {
                Some(highest) => {
                    if t > highest {
                        last = Some(t);
                        all_visible.insert((j, i));
                        left2rght.insert((j, i));
                    }
                },
                None => {
                    last = Some(t);
                    all_visible.insert((j, i));
                    left2rght.insert((j, i));
                },
            }
        }
    }

    // println!("from left to right: {:?}", left2rght);


    let mut rght2left = HashSet::new();

    // from right to left
    for j in 0..forest.len() {
        let row = &forest[j];
        let mut last = None;

        for i in (0..row.len()).rev() {
            let t = row[i];
            match last {
                Some(highest) => {
                    if t > highest {
                        last = Some(t);
                        all_visible.insert((j, i));
                        rght2left.insert((j, i));
                    }
                },
                None => {
                    last = Some(t);
                    all_visible.insert((j, i));
                    rght2left.insert((j, i));
                },
            }
        }
    }

    // println!("from right to left: {:?}", rght2left);

    let mut top2bottom = HashSet::new();

    // from top to bottom
    for i in 0..forest.len() {
        let mut last = None;
        for j in 0..forest.len() {
            let t = forest[j][i];
            match last {
                Some(num) => {
                    if t > num {
                        last = Some(t);
                        all_visible.insert((j, i));
                        top2bottom.insert((j, i));
                    }
                },
                None => {
                    last = Some(t);
                    all_visible.insert((j, i));
                    top2bottom.insert((j, i));
                },
            }
        }
    }

    // println!("top2bottom: {:?}", top2bottom);

    let mut bottom2top = HashSet::new();

    // from bottom to top
    for i in 0..forest.len() {
        let mut last = None;
        for j in (0..forest.len()).rev() {
            let t = forest[j][i];
            match last {
                Some(num) => {
                    if t > num {
                        last = Some(t);
                        all_visible.insert((j, i));
                        bottom2top.insert((j, i));
                    }
                },
                None => {
                    last = Some(t);
                    all_visible.insert((j, i));
                    bottom2top.insert((j, i));
                },
            }
        }
    }

    // println!("bottom2top: {:?}", bottom2top);

    println!("all_visible: {}", all_visible.len());

    let mut max_score = 0;
    for row in 0..forest.len() {
        for column in 0..forest[row].len() {
            let score = scenic_score(&forest, row, column);
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("max_score: {}", max_score);

    Ok(())
}


fn scenic_score(forest: &Vec<Vec<u32>>, row: usize, column: usize) -> usize {

    let height = forest[row][column];

    // up
    let mut score_up = 0;
    for r in (0..row).rev() {
        score_up += 1;
        if forest[r][column] >= height {
            break;
        }
    }
    // down
    let mut score_down = 0;
    for r in row+1..forest.len() {
        score_down += 1;
        if forest[r][column] >= height {
            break;
        }
    }
    // left
    let mut score_left = 0;
    for c in (0..column).rev() {
        score_left += 1;
        if forest[row][c] >= height {
            break;
        }
    }
    // right
    let mut score_right = 0;
    for c in column+1..forest.len() {
        score_right += 1;
        if forest[row][c] >= height {
            break;
        }
    }

    score_up * score_down * score_left * score_right
}


#[test]
fn test() {
    let x: u32 = 0;
    for i in x+1 .. 8 {
        println!("i: {}", i);
    }
}
