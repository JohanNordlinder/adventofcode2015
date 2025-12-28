use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[path = "day2/part1.rs"]
mod day2part1;
#[path = "day2/part2.rs"]
mod day2part2;
#[path = "day3/part1.rs"]
mod day3part1;
#[path = "day3/part2.rs"]
mod day3part2;
#[path = "day4/part1.rs"]
mod day4part1;
#[path = "day5/part1.rs"]
mod day5part1;
#[path = "day5/part2.rs"]
mod day5part2;

fn read_lines(filename: &str) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return BufReader::new(file).lines();
}

fn main() {
    //day2();
    //day3();
    //day4();
    day5();
}

fn day2() {
    let lines: Vec<String> = read_lines("day2.txt").map(|x| x.unwrap()).collect();
    println!("Day 2");
    day2part1::run(&lines);
    day2part2::run(&lines);
}

fn day3() {
    let lines: Vec<String> = read_lines("day3.txt").map(|x| x.unwrap()).collect();
    println!("Day 3");
    day3part1::run(&">".to_string());
    day3part1::run(&"^>v<".to_string());
    day3part1::run(&"^v^v^v^v^v".to_string());
    day3part1::run(&lines.first().unwrap());

    day3part2::run(&"^v".to_string());
    day3part2::run(&"^>v<".to_string());
    day3part2::run(&"^v^v^v^v^v".to_string());
    day3part2::run(&lines.first().unwrap());
}

fn day4() {
    println!("Day 4, part 1");
    assert_eq!(day4part1::run(&"abcdef".to_string(), 5), "609043", "Test 1 failed!");
    assert_eq!(day4part1::run(&"pqrstuv".to_string(), 5), "1048970", "Test 2 failed!");
    println!("Answer: {}", day4part1::run(&"bgvyzdsv".to_string(), 5));
    println!("Day 4, part 2");
    println!("Answer: {}", day4part1::run(&"bgvyzdsv".to_string(), 6));
}

fn day5() {
    println!("Day 5, part 1");
    assert_eq!(day5part1::naughty_or_nice(&"ugknbfddgicrmopn".to_string()), day5part1::NaughtyOrNice::Nice, "Test 1 failed!");
    assert_eq!(day5part1::naughty_or_nice(&"aaa".to_string()), day5part1::NaughtyOrNice::Nice, "Test 2 failed!");
    assert_eq!(day5part1::naughty_or_nice(&"jchzalrnumimnmhp".to_string()), day5part1::NaughtyOrNice::Naughty, "Test 3 failed!");
    assert_eq!(day5part1::naughty_or_nice(&"haegwjzuvuyypxyu".to_string()), day5part1::NaughtyOrNice::Naughty, "Test 4 failed!");
    assert_eq!(day5part1::naughty_or_nice(&"dvszwmarrgswjxmb".to_string()), day5part1::NaughtyOrNice::Naughty, "Test 5 failed!");
    let lines: Vec<String> = read_lines("day5.txt").map(|x| x.unwrap()).collect();
    println!("Answer: {}", day5part1::run(&lines));
    println!("Day 5, part 2");
    assert_eq!(day5part2::naughty_or_nice(&"qjhvhtzxzqqjkmpb".to_string()), day5part2::NaughtyOrNice::Nice, "Test 1 failed!");
    assert_eq!(day5part2::naughty_or_nice(&"xxyxx".to_string()), day5part2::NaughtyOrNice::Nice, "Test 2 failed!");
    assert_eq!(day5part2::naughty_or_nice(&"uurcxstgmygtbstg".to_string()), day5part2::NaughtyOrNice::Naughty, "Test 3 failed!");
    assert_eq!(day5part2::naughty_or_nice(&"ieodomkazucvgmuy".to_string()), day5part2::NaughtyOrNice::Naughty, "Test 4 failed!");
    println!("Answer: {}", day5part2::run(&lines));
}
