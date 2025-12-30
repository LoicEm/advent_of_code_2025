use anyhow::Result;
use std::{fs, num::ParseIntError};
use thiserror::Error;

#[cfg(test)]
mod tests;

#[derive(Debug)]
enum Operator {
    Addition,
    Multiplication,
}

impl TryFrom<&str> for Operator {
    type Error = OperatorParsingError;
    fn try_from(value: &str) -> Result<Self, OperatorParsingError> {
        match value {
            "+" => Ok(Self::Addition),
            "*" => Ok(Self::Multiplication),
            _ => Err(OperatorParsingError::InvalidOperatorCharacter(
                value.to_string(),
            )),
        }
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        v
    } else {
        let len = v[0].len();
        let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<T>>()
            })
            .collect()
    }
}

#[derive(Debug, Error)]
enum OperatorParsingError {
    #[error("Expected + or *, got {0}")]
    InvalidOperatorCharacter(String),
}

#[derive(Debug)]
struct SquidProblem {
    figures: Vec<usize>,
    operator: Operator,
}

#[derive(Debug, Error)]
enum SquidProblemParsingError {
    #[error("Invalid operator instruction")]
    InvalidOperatorInstruction(#[from] OperatorParsingError),
    #[error("Invalid figures instructions")]
    InvalidFiguresInstructions(#[from] ParseIntError),
    #[error("Empty instruction string")]
    EmptyInstruction,
}

impl SquidProblem {
    fn solve(&self) -> usize {
        match self.operator {
            Operator::Addition => self.figures.iter().sum(),
            Operator::Multiplication => self.figures.iter().product(),
        }
    }
}

// Get the split column index based on the operator line
fn get_split_columns_index(operator_line: &String) -> Vec<usize> {
    let mut output = Vec::new();
    for (index, character) in operator_line.chars().enumerate() {
        if (index > 0) & ((character == '+') | (character == '*')) {
            //then the split column is just before
            output.push(index - 1)
        }
    }
    output
}

fn split_problems_line(line: &str, indexes_to_split: &Vec<usize>) -> Vec<String> {
    let mut output = Vec::new();
    let mut start_index = 0;
    for &end_index in indexes_to_split.iter() {
        output.push(line[start_index..end_index].to_string());
        start_index = end_index + 1;
    }
    // Push the last column
    output.push(line[start_index..line.len()].to_string());
    output
}

//Parse the problems as defined in the first part of the problem
fn parse_first(instructions: &Vec<String>) -> Result<SquidProblem, SquidProblemParsingError> {
    if instructions.is_empty() {
        Err(SquidProblemParsingError::EmptyInstruction)
    } else {
        let figures = instructions[..instructions.len() - 1]
            .iter()
            .map(|figure| {
                figure
                    .trim()
                    .parse::<usize>()
                    .expect("The first instructions to be only figures")
            })
            .collect();
        let operator = Operator::try_from(instructions.last().unwrap().as_str().trim())?;
        Ok(SquidProblem { figures, operator })
    }
}

fn parse_second(instructions: &Vec<String>) -> Result<SquidProblem, SquidProblemParsingError> {
    let max_significant_numbers = instructions[..instructions.len() - 1]
        .iter()
        .map(|figure| figure.len())
        .max()
        .expect("The instructions to not be empty");
    let padded_instructions = instructions[..instructions.len() - 1]
        .iter()
        .map(|figure| {
            format!("{:width$}", figure, width = max_significant_numbers)
                .chars()
                .collect()
        })
        .collect();
    let instructions_in_squid_language = transpose(padded_instructions);
    let figures = instructions_in_squid_language
        .into_iter()
        .map(|row| {
            row.into_iter()
                .collect::<String>()
                .trim()
                .parse::<usize>()
                .expect("The instructions to be correctly parsed")
        })
        .collect();

    let operator = Operator::try_from(instructions.last().unwrap().as_str().trim())?;
    Ok(SquidProblem { figures, operator })
}

pub fn main() {
    let input_lines = fs::read_to_string("data/day_6.txt")
        .expect("The input file to be correctly read")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    let split_indexes = get_split_columns_index(
        input_lines
            .last()
            .expect("The instructions to not be empty"),
    );
    let input_lines = input_lines
        .iter()
        .map(|line| split_problems_line(line, &split_indexes))
        .collect::<Vec<Vec<String>>>();
    let input = transpose(input_lines);
    let first_problems = input
        .iter()
        .map(|instructions| parse_first(instructions).expect("The problem to be correctly parsed"))
        .collect::<Vec<SquidProblem>>();
    let first_solved_problems: usize = first_problems.iter().map(|problem| problem.solve()).sum();
    dbg!(first_solved_problems);
    let second_problems = input
        .iter()
        .map(|instructions| parse_second(instructions).expect("The problem to be correctly parsed"))
        .collect::<Vec<SquidProblem>>();
    let second_problems_solved: usize = second_problems.iter().map(|problem| problem.solve()).sum();
    dbg!(second_problems_solved);
}
