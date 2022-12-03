use std::{path::PathBuf, io::{BufReader, BufRead, Lines}, fs::File, cmp::Ordering::{self, *}};


/// A, X -> Rock, score 1  
/// B, Y -> Paper, score 2  
/// C, Z -> Scissors, score 3  
/// lost -> 0  
/// draw -> 3  
/// won  -> 6  
#[derive(PartialEq, Debug, Clone)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}


/// Lost -> score 0  
/// Draw -> score 3  
/// Win  -> score 6  
#[derive(Debug)]
enum GameResult {
    Lose,
    Draw,
    Win,
}

impl GameResult {
    fn get_score(&self) -> u64 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl RockPaperScissors {
    fn get_score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn calc_game_result(&self, opponent: &Self) -> GameResult {
        match self.partial_cmp(opponent) {
            Some(order) => {
                match order {
                    Ordering::Less => GameResult::Lose,
                    Ordering::Equal => GameResult::Draw,
                    Ordering::Greater => GameResult::Win,
                }
            },
            None => unreachable!(),
        }
    }

    fn generate_winner(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
    fn generate_failer(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

impl PartialOrd for RockPaperScissors {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Self::Rock => {
                match other {
                    Self::Rock => Some(Ordering::Equal),
                    Self::Paper => Some(Ordering::Less),
                    Self::Scissors => Some(Ordering::Greater),
                }
            },
            Self::Paper => {
                match other {
                    Self::Rock => Some(Ordering::Greater),
                    Self::Paper => Some(Ordering::Equal),
                    Self::Scissors => Some(Ordering::Less),
                }
            },
            Self::Scissors => {
                match other {
                    Self::Rock => Some(Ordering::Less),
                    Self::Paper => Some(Ordering::Greater),
                    Self::Scissors => Some(Ordering::Equal),
                }
            },
        }
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }
}

impl From<&str> for RockPaperScissors {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!()
        }
    }
}

/// part2 
/// X -> Lost  
/// Y -> Draw  
/// Z -> Win  
impl From<&str> for GameResult {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!()
        }
    }
}


pub fn day2(path: &PathBuf) -> std::io::Result<()> {
    let mut iter = BufReader::new(
        File::open(path)?
    )
    .lines()
    .into_iter();

    let mut sum_score = 0u64;
    loop {
        match get_one_score(&mut iter) {
            Some(score) => {
                sum_score += score;
            },
            None => break,
        }
    }

    println!("day2 part1 sum_score: {}", sum_score);

    part2(&path)?;

    Ok(())
}


/// part2 
/// X -> Lost  
/// Y -> Draw  
/// Z -> Win  
fn part2(path: &PathBuf) -> std::io::Result<()> {
    let lines = BufReader::new(
        File::open(path)?
    )
    .lines();

    let mut sum_score = 0u64;

    for line in lines {
        let s = line?.trim().to_string();
        let round = s.split(' ').collect::<Vec<&str>>();
        // first row is what your opponent is going to play
        let opponent: RockPaperScissors = round[0].into();
        let game_result: GameResult = round[1].into();

        let you = match game_result {
            GameResult::Draw => opponent.clone(),
            GameResult::Lose => opponent.generate_failer(),
            GameResult::Win => opponent.generate_winner(),
        };

        sum_score += game_result.get_score() + you.get_score();
    }

    println!("day2 part2 sum_score: {}", sum_score);

    Ok(())
}

fn get_one_score(iter: &mut Lines<BufReader<File>>) -> Option<u64> {
    match iter.next() {
        Some(Ok(mut line)) => {
            line = line.trim().to_string();
            let round = line.split(' ').collect::<Vec<&str>>();
            // first row is what your opponent is going to play
            let opponent: RockPaperScissors = round[0].into();
            let you: RockPaperScissors = round[1].into();

            Some(you.calc_game_result(&opponent).get_score() + you.get_score())
        },
        _ => None,
    }
}
