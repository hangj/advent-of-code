use std::{path::PathBuf, io::{BufReader, BufRead}, fs::File, ops::Sub, collections::HashSet};


enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl<T: AsRef<str>> From<T> for Direction {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "L" => Self::Left,
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => unreachable!()
        }
    }
}

struct Command {
    direction: Direction,
    step: i32,
}

impl<T: AsRef<str>> From<T> for Command {
    fn from(s: T) -> Self {
        let v = s.as_ref().split(" ").collect::<Vec<&str>>();
        Self {
            direction: v[0].into(),
            step: v[1].parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Rope {
    head: (i32, i32), // x, y
    tail: (i32, i32), // x, y
    knots: Vec<(i32, i32)>,
}

impl Rope {
    // fn new() -> Self {
    //     Self {
    //         head: (0, 0),
    //         tail: (0, 0),
    //         knots: [(0, 0); 2].into(),
    //     }
    // }

    fn with_knots(count: usize) -> Self {
        Self {
            head: (0, 0),
            tail: (0, 0),
            knots: vec![(0, 0); count],
        }
    }

    fn run_command<F: FnMut((i32, i32))>(&mut self, cmd: &Command, mut on_step: F) {

        if self.knots.is_empty() {
            return;
        }

        for _ in 0..cmd.step {
            {
                let head = &mut self.knots[0];
                match cmd.direction {
                    Direction::Left => head.0 -= 1,
                    Direction::Right => head.0 += 1,
                    Direction::Up => head.1 += 1,
                    Direction::Down => head.1 -= 1,
                }
            }

            {
                // T.H -> .TH
                // .H
                // T.
                for i in 1..self.knots.len() {
                    let diff_x = self.knots[i-1].0.abs_diff(self.knots[i].0);
                    let diff_y = self.knots[i-1].1.abs_diff(self.knots[i].1);

                    if diff_x > 1 {
                        self.knots[i].0 += if self.knots[i-1].0.sub(self.knots[i].0) > 0 { 1 } else { -1 };
                        if diff_y != 0 {
                            self.knots[i].1 = self.knots[i-1].1;
                        }
                    }
                    if diff_y > 1 {
                        self.knots[i].1 += if self.knots[i-1].1.sub(self.knots[i].1) > 0 { 1 } else { -1 };
                        if diff_x != 0 {
                            self.knots[i].0 = self.knots[i-1].0;
                        }
                    }
                }

                on_step(self.knots.last().unwrap().to_owned());
            }
        }
    }
}


pub fn day9(path: &PathBuf) -> std::io::Result<()> {
    let lines = BufReader::new(File::open(path)?).lines();

    let mut rope = Rope::with_knots(2);
    let mut rope10 = Rope::with_knots(10);

    let mut positions = HashSet::new();
    let mut positions10 = HashSet::new();

    for line in lines {
        match line {
            Ok(line) => {
                let cmd: Command = line.into();
                rope.run_command(&cmd, |pos| {
                    positions.insert(pos);
                });

                rope10.run_command(&cmd, |pos| {
                    positions10.insert(pos);
                });
            },
            Err(_) => break,
        }
    }

    println!("size: {}", positions.len());
    println!("size: {}", positions10.len());

    Ok(())
}


fn cb_test<F>(mut cb: F) where F: FnMut(i32) {
    cb(1);
}

#[test]
fn test() {
    let mut sum = 0;
    for _ in 0..10 {
        cb_test(|x| {
            // println!("x: {}", x);
            sum += x;
        });
    }
    println!("sum: {}", sum);

    let vec = vec![2; 0];
    println!("{:#?}", vec);
}
