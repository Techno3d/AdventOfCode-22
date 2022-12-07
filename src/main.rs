mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
    } else if day == "5" {
        day5::day5(&info);
        day5::day5_part2(&info);
    } else if day == "6" {
        day6::day6(&info);
    }
}

