use std::cmp::PartialEq;
use regex::Regex;
use crate::day5part1::NaughtyOrNice::{Naughty, Nice};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum NaughtyOrNice {
    Naughty,
    Nice,
}

pub fn naughty_or_nice(key: &String) -> NaughtyOrNice {
    return if Regex::new(r"ab|cd|pq|xy").unwrap().is_match(key) {
        Naughty
    } else if key
        .chars()
        .filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(&c))
        .count() >= 3 && contains_repeated_char(key) {
        Nice
    } else {
        Naughty
    }
}

fn contains_repeated_char(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }
    false
}

pub fn run(strings: &Vec<String>) -> usize {
    return strings.iter().filter(|x| naughty_or_nice(x) == NaughtyOrNice::Nice).count();
}