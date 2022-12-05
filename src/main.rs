mod day1;
mod day2;
mod day3;
mod day4;

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
    } else if day == "3" {
        day3::day3(&info);
        day3::day3_part2(&info);
    } else if day == "4" {
        day4::day4(&info);
    }
}

