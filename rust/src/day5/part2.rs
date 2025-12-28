use std::cmp::PartialEq;
use crate::day5part2::NaughtyOrNice::{Naughty, Nice};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum NaughtyOrNice {
    Naughty,
    Nice,
}

pub fn naughty_or_nice(key: &String) -> NaughtyOrNice {
    if has_repeating_char_with_another_char_in_between(key) && has_repeating_pair(key) {
        Nice
    } else {
        Naughty
    }
}

fn has_repeating_char_with_another_char_in_between(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }
    false
}

fn has_repeating_pair(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 1 {
        let first = chars[i].to_string();
        let second = chars[i + 1].to_string();
        if s[..i].contains(&format!("{}{}", first, second)) {
            return true;
        }
    }
    false
}

pub fn run(strings: &Vec<String>) -> usize {
    return strings.iter().filter(|x| naughty_or_nice(x) == NaughtyOrNice::Nice).count();
}