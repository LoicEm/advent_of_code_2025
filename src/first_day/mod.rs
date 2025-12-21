use anyhow::Result;
use std::{fs, num::ParseIntError};
use thiserror::Error;

enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests;

#[derive(Error, Debug)]
enum InstructionParsingError {
    #[error("Invalid direction, expected L or R, got {0}")]
    InvalidDirection(String),
    #[error("Invalid distance")]
    InvalidDistance(#[from] ParseIntError),
}

struct RotationInstruction {
    direction: Direction,
    distance: usize,
}

impl RotationInstruction {
    fn new(instruction_string: &str) -> Result<Self, InstructionParsingError> {
        let direction_string = &instruction_string[0..1];
        let direction = match direction_string {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(InstructionParsingError::InvalidDirection(
                direction_string.to_string(),
            )),
        }?;
        let distance: usize = instruction_string[1..].parse()?;
        Ok(RotationInstruction {
            direction,
            distance,
        })
    }
}

#[derive(Debug)]
struct Dial {
    position: usize,
    n_times_landed_on_zero: usize,
    n_times_passed_on_zero: usize,
}

impl Dial {
    fn new(starting_position: usize) -> Self {
        Self {
            position: starting_position,
            n_times_landed_on_zero: 0,
            n_times_passed_on_zero: 0,
        }
    }

    fn rotate(&mut self, instruction: RotationInstruction) {
        // Use a signed integer as the absolute position can go below 0 or above 100
        let mut absolute_position = self.position as i16;
        match instruction.direction {
            Direction::Left => absolute_position -= instruction.distance as i16,
            Direction::Right => absolute_position += instruction.distance as i16,
        };
        // Actually not sure this works that way
        if absolute_position <= 0 {
            let mut pass_on_zero_for_the_rotation = (absolute_position.abs() / 100) as usize;
            // If starting position is different from 0
            if self.position != 0 {
                pass_on_zero_for_the_rotation += 1
            };
            self.n_times_passed_on_zero += pass_on_zero_for_the_rotation
        };
        if absolute_position >= 100 {
            self.n_times_passed_on_zero += (absolute_position / 100) as usize
        };

        // Update final position
        self.position = ((100 + (absolute_position % 100)) % 100) as usize;
        if self.position == 0 {
            self.n_times_landed_on_zero += 1;
        };
    }
}

pub fn main() {
    let mut dial = Dial::new(50);
    fs::read_to_string("data/day_1.txt")
        .expect("The input to be correctly read.")
        .lines()
        .map(|line| RotationInstruction::new(line).expect("the instruction to be correctly parsed"))
        .for_each(|instruction| dial.rotate(instruction));
    dbg!(dial);
}
