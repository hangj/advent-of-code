mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;



fn main() {
    let args = std::env::args()
        .into_iter()
        .collect::<Vec<String>>();
    
    if args.len() < 2 {
        println!("usage: {} filename", args[0]);
        return;
    }

    let filename = &args[1];

    // day1::day1(filename.into()).unwrap();
    // day2::day2(&filename.into()).unwrap();
    // day3::day3(&filename.into()).unwrap();
    // day4::day4(&filename.into()).unwrap();
    // day5::day5(&filename.into()).unwrap();
    // day6::day6(&filename.into()).unwrap();
    // day7::day7(&filename.into()).unwrap();
    // day8::day8(&filename.into()).unwrap();
    day9::day9(&filename.into()).unwrap();
}
