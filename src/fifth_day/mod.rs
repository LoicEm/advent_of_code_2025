use anyhow::Result;
use std::fs;

#[cfg(test)]
mod tests;

use crate::second_day::InputIdRange;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
struct ProcessingIdRange {
    start: usize,
    end: usize,
}

struct IngredientDatabase {
    id_ranges: Vec<ProcessingIdRange>,
    ids: Vec<usize>,
}

impl IngredientDatabase {
    fn from_path(path: &str) -> Result<Self> {
        let mut id_ranges = Vec::new();
        let mut ids = Vec::new();

        let mut range_closure = |line: &str| -> Result<()> {
            let input_range = InputIdRange::new(line)?;
            id_ranges.push(ProcessingIdRange::from(input_range));
            Ok(())
        };
        let mut id_closure = |line: &str| -> Result<()> {
            let id = line.parse::<usize>()?;
            ids.push(id);
            Ok(())
        };

        enum ProcessingType {
            Range,
            Id,
        }

        let mut processing_method = ProcessingType::Range;
        for line in fs::read_to_string(path)
            .expect("The input to be correctly read")
            .lines()
        {
            if line.is_empty() {
                processing_method = ProcessingType::Id;
                continue;
            };
            match processing_method {
                ProcessingType::Range => range_closure(line)?,
                ProcessingType::Id => id_closure(line)?,
            };
        }
        Ok(IngredientDatabase { id_ranges, ids })
    }
}

impl From<InputIdRange> for ProcessingIdRange {
    fn from(value: InputIdRange) -> Self {
        ProcessingIdRange {
            start: value.start,
            end: value.end,
        }
    }
}

struct SpoiledAndFreshIngredients {
    #[allow(dead_code)]
    spoiled: Vec<usize>,
    fresh: Vec<usize>,
}

fn separate_spoiled_and_fresh_ingredients(
    database: &IngredientDatabase,
) -> Result<SpoiledAndFreshIngredients> {
    let mut fresh = Vec::new();
    let mut spoiled = Vec::new();

    let mut ranges = database.id_ranges.clone();
    let mut ids = database.ids.clone();
    ranges.sort();
    ids.sort();

    let mut iter_ranges = ranges.iter();
    let mut current_range = iter_ranges.next();
    let mut iter_ids = ids.iter();
    let mut current_id = iter_ids.next();
    while let Some(id) = current_id {
        match current_range {
            None => {
                spoiled.push(id.clone());
                current_id = iter_ids.next()
            }
            Some(range) => {
                if id < &range.start {
                    spoiled.push(id.clone());
                    current_id = iter_ids.next();
                } else if id <= &range.end {
                    fresh.push(id.clone());
                    current_id = iter_ids.next();
                } else {
                    current_range = iter_ranges.next()
                }
            }
        };
    }
    Ok(SpoiledAndFreshIngredients { spoiled, fresh })
}

fn count_number_of_fresh_ingredients(fresh_ingredient_ranges: &Vec<ProcessingIdRange>) -> usize {
    let mut n_fresh_ingredients = 0;
    let mut ranges = fresh_ingredient_ranges.clone();
    ranges.sort();

    let mut iter_ingredients_range = ranges.into_iter();
    let mut previous_range = ProcessingIdRange { start: 0, end: 0 };
    let mut current_range = iter_ingredients_range.next();
    while let Some(valid_current_range) = current_range {
        if previous_range.end >= valid_current_range.end {
            // current_range is contained within previous_range
            current_range = iter_ingredients_range.next();
            continue;
        } else if previous_range.end >= valid_current_range.start {
            let valid_current_range = ProcessingIdRange {
                start: previous_range.end + 1,
                end: valid_current_range.end,
            };
            n_fresh_ingredients += &valid_current_range.end - &valid_current_range.start + 1;
            previous_range = valid_current_range;
        } else {
            n_fresh_ingredients += &valid_current_range.end - &valid_current_range.start + 1;
            previous_range = valid_current_range;
        }
        current_range = iter_ingredients_range.next();
    }
    n_fresh_ingredients
}

pub fn main() {
    let database =
        IngredientDatabase::from_path("data/day_5.txt").expect("Input to be correctly processed");
    let ingredients = separate_spoiled_and_fresh_ingredients(&database)
        .expect("The ingredients to be correctly separated");
    dbg!(ingredients.fresh.len());
    dbg!(count_number_of_fresh_ingredients(&database.id_ranges));
}
