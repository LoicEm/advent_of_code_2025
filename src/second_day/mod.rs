use anyhow::Result;
use itertools::Itertools;
use std::{fs, iter::successors, num::ParseIntError};
use thiserror::Error;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct InputIdRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Error, Debug)]
pub enum InvalidInputError {
    #[error("Could not find start and date from {0}")]
    StartAndEndNotParsed(String),
    #[error("Could not parse int from {0}")]
    InvalidBoundary(#[from] ParseIntError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl InputIdRange {
    pub fn new(input_string: &str) -> Result<Self, InvalidInputError> {
        let split_input = input_string.trim().split("-").collect::<Vec<&str>>();
        if let [start, end] = split_input[..] {
            Ok(InputIdRange {
                start: start.parse::<usize>()?,
                end: end.parse::<usize>()?,
            })
        } else {
            Err(InvalidInputError::StartAndEndNotParsed(
                input_string.to_string(),
            ))
        }
    }
}

// To determine if a number is a silly pattern, it must:
// - Be of even number of character (otherwise it cannot be the same pattern repeated twice)
// - For a number of length 2n, be a multiple of 10^n + 1 (e.g 1212 is 12*101, 134134 is 134 * 1001)
// Based on this, we try to solve the challenge without ever converting the value to string

fn number_is_silly(number: usize) -> bool {
    let number_length = successors(Some(number), |&n| (n >= 10).then(|| n / 10)).count() as u32;
    if number_length % 2 != 0 {
        false
    } else {
        let ten: usize = 10;
        let diviser = ten.pow(number_length / 2) + 1;
        number % diviser == 0
    }
}

fn get_silly_numbers(input_range: &InputIdRange, silly_detector: fn(usize) -> bool) -> Vec<usize> {
    let mut output = Vec::new();
    for i in input_range.start..input_range.end {
        if silly_detector(i) {
            output.push(i);
        }
    }
    output
}

// Part two
// There are two cases:
// - If the number is repeated an even number of times (or if the number is 1 char long): our first use case already handles that.
// - If the number is repeated an odd number of times (e.g 121212) 120000 + 1200 + 12 = 12*(10000 + 100 + 1) = 12 * 10101
// 123123123 = 123 * 1001001
fn number_is_silly_part_two(number: usize) -> bool {
    let number_length = successors(Some(number), |&n| (n >= 10).then(|| n / 10)).count() as u32;
    if number_length == 1 {
        false
    } else if number_length % 2 != 0 {
        let all_ones_char = (0..number_length).map(|_| '1').collect::<String>();
        let all_ones = all_ones_char
            .parse::<usize>()
            .expect("All ones to be parseable into usize.");
        (number % all_ones == 0) || ((number_length == 9) & (number % 1001001 == 0))
        // Number is divisible by 111... (1 number_lenght times)
        // OR Numbers in the input are at most 11 characters long, so only other case
        // for odd numbers is a 3 character number repeated 3 times.
    } else {
        let ten: usize = 10;
        let diviser = ten.pow(number_length / 2) + 1;
        // Number repeated an even number of time
        (number % diviser == 0)
            // OR 2 characters repeated 3 or 5 times
            || ((number_length == 6) & (number % 10101 == 0))
            || ((number_length == 10) & (number % 101010101 == 0))
    }
}

pub fn main() {
    let input_ranges = fs::read_to_string("data/day_2.txt")
        .expect("The input to be correctly read")
        .split(",")
        .map(|row| InputIdRange::new(row).expect("The input to be correctly parsed"))
        .collect::<Vec<InputIdRange>>();

    let first_silly_number_sum: usize = input_ranges
        .iter()
        .map(|input_range| get_silly_numbers(input_range, number_is_silly))
        .flatten()
        .sum();
    dbg!(first_silly_number_sum);

    let second_silly_number_sum = input_ranges
        .iter()
        .map(|input_range| get_silly_numbers(input_range, number_is_silly_part_two))
        .flatten()
        .unique()
        .sum::<usize>();
    dbg!(second_silly_number_sum);
}
