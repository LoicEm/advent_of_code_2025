use std::{cmp::Ordering, collections::HashSet, fmt::Display, fs, iter::zip};

use itertools::Itertools;

#[cfg(test)]
mod tests;

struct TachyonManifold {
    starting_position: usize,
    splitter_lines: Vec<HashSet<usize>>,
}

#[derive(Debug)]
struct TachyonManifoldResult {
    n_splits: usize,
}

impl Display for TachyonManifoldResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Number of splits: {}", self.n_splits)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct QuanticTachyonManifoldResult {
    n_timelines: usize,
}

impl Display for QuanticTachyonManifoldResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Number of timelines: {}", self.n_timelines)
    }
}

impl TachyonManifold {
    fn run(&self) -> TachyonManifoldResult {
        let mut beam = TachyonBeams {
            current_position: HashSet::from([self.starting_position]),
            n_splits: 0,
        };
        self.splitter_lines
            .iter()
            .for_each(|splitter_line| beam = beam.advance(splitter_line));
        TachyonManifoldResult {
            n_splits: beam.n_splits,
        }
    }
    fn run_quantic(&self) -> QuanticTachyonManifoldResult {
        let mut beams = vec![QuanticTachyonBeams {
            current_position: HashSet::from([self.starting_position]),
            n_previous_timelines: 1,
        }];
        self.splitter_lines.iter().for_each(|splitter_line| {
            beams = beams
                .clone()
                .iter()
                .map(|beam| beam.advance(splitter_line))
                .flatten()
                // Regroup all the timelines of a line
                .chunk_by(|beam| beam.current_position.clone())
                .into_iter()
                .map(|(position, beams)| {
                    let n_previous_timelines = beams.map(|beam| beam.n_previous_timelines).sum();
                    QuanticTachyonBeams {
                        current_position: position.clone(),
                        n_previous_timelines,
                    }
                })
                .collect();
        });
        QuanticTachyonManifoldResult {
            n_timelines: beams
                .into_iter()
                .map(|final_beam| final_beam.n_previous_timelines)
                .sum(),
        }
    }
}

//Represent the tachyon beams
//Todo: refactor to use it even for the quantic use case
#[derive(Debug, PartialEq, Eq)]
struct TachyonBeams {
    current_position: HashSet<usize>,
    n_splits: usize,
}

impl TachyonBeams {
    fn advance(&self, splitter_line: &HashSet<usize>) -> Self {
        let mut output_beam = HashSet::new();
        let mut split_on_line = 0;
        self.current_position.iter().for_each(|position| {
            if splitter_line.contains(position) {
                output_beam.insert(*position - 1);
                output_beam.insert(*position + 1);
                split_on_line += 1
            } else {
                output_beam.insert(*position);
            }
        });
        TachyonBeams {
            current_position: output_beam,
            n_splits: self.n_splits + split_on_line,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct QuanticTachyonBeams {
    current_position: HashSet<usize>,
    n_previous_timelines: usize,
}

impl Ord for QuanticTachyonBeams {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ordering = Ordering::Equal;
        for (a, b) in zip(
            self.current_position.iter().sorted(),
            other.current_position.iter().sorted(),
        ) {
            match a.cmp(b) {
                Ordering::Greater => {
                    ordering = Ordering::Greater;
                    break;
                }
                Ordering::Less => {
                    ordering = Ordering::Less;
                    break;
                }
                Ordering::Equal => {
                    ordering = Ordering::Equal;
                    continue;
                }
            }
        }
        ordering
    }
}

impl PartialOrd for QuanticTachyonBeams {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl QuanticTachyonBeams {
    fn split(&self, on: usize) -> Vec<Self> {
        let mut left_split: HashSet<usize> = self
            .current_position
            .union(&HashSet::from([on - 1]))
            .copied()
            .collect();
        left_split.remove(&on);
        let mut right_split: HashSet<usize> = self
            .current_position
            .union(&HashSet::from([on + 1]))
            .copied()
            .collect();
        right_split.remove(&on);
        vec![
            QuanticTachyonBeams {
                current_position: left_split,
                n_previous_timelines: self.n_previous_timelines,
            },
            QuanticTachyonBeams {
                current_position: right_split,
                n_previous_timelines: self.n_previous_timelines,
            },
        ]
    }

    fn advance(&self, splitter_line: &HashSet<usize>) -> Vec<Self> {
        let mut output_beams = vec![QuanticTachyonBeams {
            current_position: HashSet::new(),
            n_previous_timelines: self.n_previous_timelines,
        }];
        self.current_position.iter().for_each(|position| {
            output_beams = output_beams
                .clone()
                .into_iter()
                .map(|mut possible_beam| {
                    if splitter_line.contains(position) {
                        possible_beam.split(*position)
                    } else {
                        possible_beam.current_position.insert(*position);
                        vec![possible_beam]
                    }
                })
                .flatten()
                .collect::<Vec<QuanticTachyonBeams>>();
        });
        output_beams
    }
}

pub fn main() {
    let input = fs::read_to_string("data/day_7.txt")
        .expect("The input to be correctly read")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    let starting_position = input
        .first()
        .expect("The input to not be empty")
        .chars()
        .position(|c| c == 'S')
        .expect("The first line to contain a starting point");
    let splitter_lines = input[1..]
        .iter()
        .filter(|&line| line.contains("^"))
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(index, c)| if c == '^' { Some(index) } else { None })
                .collect::<HashSet<usize>>()
        })
        .collect::<Vec<HashSet<usize>>>();
    let manifold = TachyonManifold {
        starting_position,
        splitter_lines,
    };
    let result = manifold.run();
    println!("The manifold has the following stats {}", result);
    let result_quantic = manifold.run_quantic();
    println!(
        "The manifold has the following quantic stats {}",
        result_quantic
    );
}
