use std::{path::PathBuf, io::{BufReader, BufRead, Lines}, fs::File};





pub fn day5(path: &PathBuf) -> std::io::Result<()> {
    let mut iter = BufReader::new(File::open(path)?).lines().into_iter();

    let matrix = get_matrix(&mut iter).unwrap();
    let mut matrix = rotate_matrix(matrix);
    let mut matrix_v2 = matrix.clone();
    // println!("{:#?}", matrix);

    let commands = read_commands(&mut iter);
    // println!("{:#?}", commands);

    for cmd in &commands {
        run_command(&mut matrix, &cmd);
    }

    for cmd in &commands {
        run_command_part2(&mut matrix_v2, &cmd);
    }

    let result = matrix.into_iter()
        .map(|v| *v.last().unwrap())
        .collect::<String>();
    println!("day5 part1 crates on top of each stack: {}", result);

    let result = matrix_v2.into_iter()
        .map(|v| *v.last().unwrap())
        .collect::<String>();
    println!("day5 part2 crates on top of each stack: {}", result);

    Ok(())
}


fn get_matrix(iter: &mut Lines<BufReader<File>>) -> Option<Vec<Vec<char>>> {
    let mut vec_all = Vec::new();
    loop {
        match iter.next() {
            Some(Ok(line)) => {
                if line.starts_with(" ") || line.trim().is_empty() {
                    break;
                }

                let mut vec = Vec::new();

                // read 3 char
                let mut idx = 0;
                while idx < line.len() {
                    let s = &line[idx+1..idx+2];
                    vec.push(s.chars().next().unwrap());
                    idx += 4;
                }
                
                vec_all.push(vec);
            },
            _ => break,
        }
    }
    vec_all.reverse();
    Some(vec_all)
}

fn rotate_matrix(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut vec_all: Vec<Vec<char>> = std::vec::from_elem(Vec::new(), matrix[0].len());

    for vec in matrix.into_iter() {
        let mut idx = 0;
        for c in vec.into_iter() {
            if !c.is_whitespace() { vec_all[idx].push(c); }
            idx += 1;
        }
    }

    vec_all
}


#[derive(Debug)]
struct Command {
    count: usize,
    from: usize,
    to: usize,
}

fn read_commands(iter: &mut Lines<BufReader<File>>) -> Vec<Command> {
    let mut vec_all = Vec::new();
    loop {
        match iter.next() {
            Some(Ok(line)) => {
                if line.trim().is_empty() { continue; }
                let vec = line.split(" ").collect::<Vec<&str>>();
                vec_all.push(Command {
                    count: vec[1].parse().unwrap(),
                    from: vec[3].parse().unwrap(),
                    to: vec[5].parse().unwrap(),
                });
            },
            _ => break,
        }
    }

    vec_all
}


fn run_command(matrix: &mut Vec<Vec<char>>, cmd: &Command) {
    let from = cmd.from - 1;
    let to = cmd.to - 1;
    for _ in 0..cmd.count {
        let item = matrix[from].pop().unwrap();
        matrix[to].push(item);
    }
}

fn run_command_part2(matrix: &mut Vec<Vec<char>>, cmd: &Command) {
    // let from = &matrix[cmd.from - 1];
    // let to = &mut matrix[cmd.to - 1];

    let (len, _idx, mut from) = {
        let from = &matrix[cmd.from - 1];
        let len = from.len();
        let idx = len - cmd.count;
        (len, idx, from[idx..len].to_owned())
    };

    // matrix[cmd.to - 1].splice(.., matrix[cmd.from - 1][idx..len]);
    matrix[cmd.to - 1].append(&mut from);
    matrix[cmd.from - 1].truncate(len - cmd.count);
}


#[test]
fn test() {
    let s = "move 6 from 1 to 7".to_string();
    let vec = s.split(" ").collect::<Vec<&str>>();
    println!("{:#?}", vec);

    let mut v1 = vec![1,2,3];
    let mut v2 = vec![4,5,6,7,8];
    v1.append(&mut v2[2..4].to_owned());

    println!("v1: {:#?}", v1);
    println!("v2: {:#?}", v2);
}
