mod day1;
mod day2;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).unwrap();
    let file = match args.get(2) {
        Some(a) => a,
        None => todo!(),
    };
    let info = fs::read_to_string(file).unwrap();

    if day == "1" {
        day1::day1(info);
    } else if day == "2" {
        day2::day2(info);
    }
}


