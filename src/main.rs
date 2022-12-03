mod day1;
mod day2;

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
    day2::day2(&filename.into()).unwrap();
}
