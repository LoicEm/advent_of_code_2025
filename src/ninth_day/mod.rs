use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::read_to_string,
};

use anyhow::Result;
use itertools::{Itertools, iproduct};
use thiserror::Error;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq, Eq)]
struct RedTile {
    x: usize,
    y: usize,
}

impl Display for RedTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RedTile<x:{}, y:{}>", self.x, self.y)
    }
}

impl From<&str> for RedTile {
    fn from(value: &str) -> Self {
        let split_str = value.split(",").take(2).collect_vec();
        Self {
            x: split_str[0]
                .parse()
                .expect("The x coordinate to represent an integer"),
            y: split_str[1]
                .parse()
                .expect("The y coordinate to represent an integer"),
        }
    }
}

fn parse_input(input: String) -> Vec<RedTile> {
    input.lines().map(RedTile::from).collect()
}

fn get_tile_combinations(tiles: &Vec<RedTile>) -> Vec<(RedTile, RedTile)> {
    iproduct!(tiles.iter().enumerate(), tiles.iter().enumerate())
        .filter_map(|((i, tile_a), (j, tile_b))| {
            if i < j {
                Some((tile_a.clone(), tile_b.clone()))
            } else {
                None
            }
        })
        .collect()
}

fn calculate_area(tile_a: &RedTile, tile_b: &RedTile) -> usize {
    let width = (tile_a.x as isize - tile_b.x as isize).abs() + 1;
    let length = (tile_a.y as isize - tile_b.y as isize).abs() + 1;
    (width * length) as usize
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct GreenInterval {
    start: usize,
    end: usize,
}

#[derive(Error, Debug)]
enum LineCreationError {
    #[error("Missing point for closing a green column")]
    MissingPointForClosingGreenColumn,
    #[error("The interval {0:?} was not found")]
    MissingInterval(GreenInterval),
    #[error("Empty input")]
    EmptyInput,
}

#[derive(Debug, Clone)]
struct LineGreenIntervals {
    current_intervals: HashSet<GreenInterval>,
    // We can accept values in the current interval but reject them in the future, when we split an interval
    next_intervals: HashSet<GreenInterval>,
}

fn check_line(line_tiles: &Vec<RedTile>) -> Result<(), LineCreationError> {
    // Check that a line     has a correct value
    if line_tiles.len() % 2 != 0 {
        Err(LineCreationError::MissingPointForClosingGreenColumn)
    } else {
        Ok(())
    }
}

impl LineGreenIntervals {
    fn new_first_line(line_tiles: &Vec<RedTile>) -> Result<Self, LineCreationError> {
        check_line(line_tiles)?;
        let mut tile_iter = line_tiles.iter().sorted_by_key(|&tile| tile.x);
        let mut intervals = HashSet::new();
        while let (Some(&RedTile { x: opening_x, .. }), Some(&RedTile { x: closing_x, .. })) =
            (tile_iter.next(), tile_iter.next())
        {
            intervals.insert(GreenInterval {
                start: opening_x,
                end: closing_x,
            });
        }
        Ok(LineGreenIntervals {
            current_intervals: intervals.clone(),
            next_intervals: intervals.clone(),
        })
    }

    fn new(
        line_tiles: &Vec<RedTile>,
        previous_line: LineGreenIntervals,
    ) -> Result<Self, LineCreationError> {
        check_line(line_tiles)?;
        let mut tile_iter = line_tiles.iter().sorted_by_key(|&tile| tile.x);
        let mut current_intervals = previous_line.clone();
        let mut next_intervals = previous_line.clone();
        while let (Some(&RedTile { x: opening_x, .. }), Some(&RedTile { x: closing_x, .. })) =
            (tile_iter.next(), tile_iter.next())
        {
            let start_belonging_interval = next_intervals.find_interval(opening_x).cloned();
            let end_belonging_interval = next_intervals.find_interval(closing_x).cloned();
            match (start_belonging_interval, end_belonging_interval) {
                (None, None) => {
                    let interval = GreenInterval {
                        start: opening_x,
                        end: closing_x,
                    };
                    current_intervals.insert(interval);
                    next_intervals.insert(interval);
                    Ok(())
                }
                //Extend the interval to the right
                (Some(opening_interval), None) => {
                    let extended_interval = GreenInterval {
                        start: opening_interval.start,
                        end: closing_x,
                    };
                    current_intervals.extend_interval(
                        &opening_interval,
                        &extended_interval.clone(),
                        true,
                    )?;
                    next_intervals.replace_value_by(
                        vec![opening_interval],
                        vec![extended_interval.clone()],
                        false,
                    )?;
                    Ok(())
                }
                //Extend the interval to the left
                (None, Some(closing_interval)) => {
                    let extended_interval = GreenInterval {
                        start: opening_x,
                        end: closing_interval.end,
                    };
                    current_intervals.replace_value_by(
                        vec![closing_interval],
                        vec![extended_interval],
                        false,
                    )?;
                    next_intervals.replace_value_by(
                        vec![closing_interval],
                        vec![extended_interval],
                        false,
                    )?;
                    Ok(())
                }
                (Some(opening_interval), Some(closing_interval)) => {
                    if opening_interval == closing_interval {
                        // Split the intervals
                        let mut intervals_to_insert = Vec::with_capacity(2);
                        if opening_interval.start != opening_x {
                            intervals_to_insert.push(GreenInterval {
                                start: opening_interval.start,
                                end: opening_x,
                            })
                        };
                        if opening_interval.end != closing_x {
                            intervals_to_insert.push(GreenInterval {
                                start: closing_x,
                                end: closing_interval.end,
                            })
                        };
                        next_intervals.replace_value_by(
                            vec![opening_interval],
                            intervals_to_insert,
                            false,
                        )?;
                        Ok(())
                    } else {
                        // Merge intervals
                        let interval_to_insert = GreenInterval {
                            start: opening_interval.start,
                            end: closing_interval.end,
                        };
                        current_intervals.replace_value_by(
                            vec![opening_interval, closing_interval],
                            vec![interval_to_insert],
                            false,
                        )?;
                        next_intervals.replace_value_by(
                            vec![opening_interval, closing_interval],
                            vec![interval_to_insert],
                            false,
                        )?;
                        Ok(())
                    }
                }
            }?;
        }
        Ok(LineGreenIntervals {
            //TODO clarify the attributes names, or split in two ?
            current_intervals: current_intervals.next_intervals.clone(),
            next_intervals: next_intervals.next_intervals.clone(),
        })
    }

    fn find_interval(&self, x: usize) -> Option<&GreenInterval> {
        self.next_intervals
            .iter()
            .find(|GreenInterval { start, end }| (*start <= x) & (x <= *end))
    }

    fn find_in_current_interval(&self, x: usize) -> Option<&GreenInterval> {
        self.current_intervals
            .iter()
            .find(|GreenInterval { start, end }| (*start <= x) & (x <= *end))
    }

    fn find_equivalent_interval(&self, interval: &GreenInterval) -> Option<&GreenInterval> {
        let start_interval_result = self.find_interval(interval.start);
        let end_interval_result = self.find_interval(interval.end);
        match (start_interval_result, end_interval_result) {
            (Some(start_interval), Some(end_interval)) => {
                if start_interval == end_interval {
                    Some(start_interval)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn replace_value_by(
        &mut self,
        to_replace: Vec<GreenInterval>,
        by: Vec<GreenInterval>,
        approximative: bool,
    ) -> Result<(), LineCreationError> {
        // replace value in next_intervals
        to_replace.iter().try_for_each(|interval| {
            let interval = if !approximative {
                interval
            } else {
                self.find_equivalent_interval(interval)
                    .ok_or(LineCreationError::MissingInterval(interval.clone()))?
            }
            .clone();
            if self.next_intervals.remove(&interval) {
                Ok(())
            } else {
                Err(LineCreationError::MissingInterval(interval.clone()))
            }
        })?;
        by.into_iter().for_each(|interval| self.insert(interval));
        Ok(())
    }

    fn insert(&mut self, value: GreenInterval) -> () {
        self.next_intervals.insert(value);
    }

    fn extend_interval(
        &mut self,
        original_interval: &GreenInterval,
        extending_interval: &GreenInterval,
        approximative: bool,
    ) -> Result<(), LineCreationError> {
        // Return the original interval with the extended value
        let original_interval = if !approximative {
            original_interval
        } else {
            self.find_equivalent_interval(original_interval).ok_or(
                LineCreationError::MissingInterval(original_interval.clone()),
            )?
        };
        let extended_interval = GreenInterval {
            start: min(original_interval.start, extending_interval.end),
            end: max(original_interval.end, extending_interval.end),
        };
        self.replace_value_by(
            vec![original_interval.clone()],
            vec![extended_interval],
            false,
        )
    }
}

#[derive(Debug)]
struct FigureIntervals {
    lines: HashMap<usize, LineGreenIntervals>,
}

impl FigureIntervals {
    fn new(tiles: &Vec<RedTile>) -> Result<Self> {
        let mut output_lines = HashMap::new();
        let mut sorted_tiles = tiles
            .clone()
            .into_iter()
            .sorted_by_key(|tile| (tile.y, tile.x))
            .chunk_by(|tile| tile.y)
            .into_iter()
            .map(|(y, iterator)| (y, iterator.collect::<Vec<RedTile>>()))
            .collect::<HashMap<usize, Vec<RedTile>>>()
            .into_iter()
            .sorted_by_key(|(y, _)| *y);

        let (first_line_index, first_line) = sorted_tiles
            .next()
            .ok_or_else(|| LineCreationError::EmptyInput)
            .and_then(|(i, tiles)| {
                LineGreenIntervals::new_first_line(&tiles).map(|line| (i, line))
            })?;

        output_lines.insert(first_line_index, first_line.clone());

        let mut previous_line = first_line.clone();
        for (i, line_tiles) in sorted_tiles {
            let line = LineGreenIntervals::new(&line_tiles, previous_line)?;
            output_lines.insert(i, line.clone());
            previous_line = line.clone();
        }

        Ok(FigureIntervals {
            lines: output_lines,
        })
    }

    fn rectangle_is_within_figure(&self, tile_a: &RedTile, tile_b: &RedTile) -> bool {
        let start_x = min(tile_a.x, tile_b.x);
        let end_x = max(tile_a.x, tile_b.x);
        let start_y = min(tile_a.y, tile_b.y);
        let end_y = max(tile_a.y, tile_b.y);
        for y in start_y..=end_y {
            if let Some(line) = self.lines.get(&y) {
                match line.find_in_current_interval(start_x) {
                    Some(interval) => {
                        if interval.end < end_x {
                            return false;
                        }
                    }
                    None => return false,
                };
            }
        }
        true
    }
}

pub fn main() {
    let input =
        parse_input(read_to_string("data/day_9.txt").expect("The data file to be readable"));
    let biggest_surface = get_tile_combinations(&input)
        .iter()
        .map(|(a, b)| calculate_area(a, b))
        .max()
        .expect("That there are some possible rectangles");
    println!("The biggest surface available is {}", biggest_surface);

    // Second part of the problem is identical, but we have to filter out the elements that are inside the expected area
    let figure = FigureIntervals::new(&input).expect("The figure to be correctly created");
    let biggest_rectangle = get_tile_combinations(&input)
        .iter()
        .sorted_by_key(|(tile_a, tile_b)| calculate_area(tile_a, tile_b))
        .rev()
        .find(|(tile_a, tile_b)| figure.rectangle_is_within_figure(tile_a, tile_b))
        .expect("There is at least one valid rectangle")
        .clone();
    println!(
        "The biggest rectangle is of area {}",
        calculate_area(&biggest_rectangle.0, &biggest_rectangle.1)
    );
}
