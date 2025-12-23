#[cfg(test)]
mod tests;

use std::fs;

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone)]
struct RollOfPaper {
    x: usize,
    y: usize,
}

impl RollOfPaper {
    fn is_adjacent(&self, other: &Self) -> bool {
        let mut conditions = (self != other) & (self.x <= other.x + 1) & (self.y <= other.y + 1);
        if other.x > 0 {
            conditions = conditions & (self.x >= other.x - 1)
        };
        if other.y > 0 {
            conditions = conditions & (self.y >= other.y - 1)
        };
        conditions
    }
}

fn brute_force_accessible_rolls(
    rolls: &Vec<RollOfPaper>,
    maximum_adjacent_rolls: usize,
) -> Vec<RollOfPaper> {
    let mut available_rolls = Vec::new();
    for roll in rolls.iter() {
        let mut n_adjacent_rolls = 0;
        let mut roll_is_available = true;
        for candidate_roll in rolls {
            if roll.is_adjacent(candidate_roll) {
                n_adjacent_rolls += 1;
                if n_adjacent_rolls > maximum_adjacent_rolls {
                    roll_is_available = false;
                    break;
                };
            }
        }
        if roll_is_available {
            available_rolls.push(roll.clone());
        };
    }
    available_rolls
}

fn remove_available_rolls(rolls: &mut Vec<RollOfPaper>, available_rolls: &Vec<RollOfPaper>) {
    rolls
        .extract_if(.., |roll| available_rolls.contains(roll))
        .for_each(drop);
}

fn count_accessible_rolls_with_iterative_removal(mut rolls: Vec<RollOfPaper>) -> usize {
    let mut total_removed_rolls = 0;
    let mut available_rolls = brute_force_accessible_rolls(&rolls, 3);
    while !available_rolls.is_empty() {
        available_rolls = brute_force_accessible_rolls(&rolls, 3);
        total_removed_rolls += available_rolls.len();
        remove_available_rolls(&mut rolls, &available_rolls);
    }
    total_removed_rolls
}

pub fn main() {
    let mut rolls = fs::read_to_string("data/day_4.txt")
        .expect("The input to be correctly read")
        .lines()
        .enumerate()
        .map(|(row_index, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(col_index, element)| {
                    (element == '@').then_some(RollOfPaper {
                        x: row_index,
                        y: col_index,
                    })
                })
                .collect::<Vec<RollOfPaper>>()
        })
        .flatten()
        .collect::<Vec<RollOfPaper>>();
    rolls.sort();
    let available_rolls = brute_force_accessible_rolls(&rolls, 3);
    dbg!(available_rolls.len());
    dbg!(count_accessible_rolls_with_iterative_removal(rolls));
}
