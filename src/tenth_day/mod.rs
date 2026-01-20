use std::{
    cmp::min,
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
    num::ParseIntError,
    time::SystemTime,
    u16,
};

use anyhow::{Result, anyhow};
use dpc_pariter::IteratorExt;
use itertools::Itertools;
use regex::Regex;
use thiserror::Error;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Button {
    index: usize,
    lights_activated: HashSet<usize>,
}

impl Button {
    fn new(index: usize, input: &str) -> Result<Self> {
        let lights_activated = input
            .split(",")
            .map(|number| number.parse::<usize>())
            .collect::<Result<HashSet<usize>, ParseIntError>>()?;
        Ok(Button {
            index,
            lights_activated,
        })
    }
}

struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Button>,
    target_joltage: Vec<u16>,
}

impl Machine {
    fn new(input: &str) -> Result<Self> {
        let regex = Regex::new(
            r"\[(?<target_lights>.+)\] (?<buttons>\([\d, \(\)]*\)) (?<voltage>\{[\d,]*\})",
        )?;
        let caps = regex.captures(input).expect("The regex to work");
        let target_lights = caps["target_lights"]
            .chars()
            .map(|light| match light {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err(anyhow!("Invalid value {}", light.to_string())),
            })
            .collect::<Result<Vec<bool>>>()?;
        let buttons = caps["buttons"]
            .split(" ")
            .enumerate()
            .map(|(i, button_str)| Button::new(i, button_str.trim_matches(['(', ')'])))
            .collect::<Result<Vec<Button>>>()?;
        let voltage = caps["voltage"]
            .trim_matches(['{', '}'])
            .split(",")
            .map(|voltage| voltage.parse::<u16>())
            .collect::<Result<Vec<u16>, ParseIntError>>()?;
        Ok(Machine {
            target_lights,
            buttons,
            target_joltage: voltage,
        })
    }

    fn find_fastest_way_to_toogle_light_panel(&self) -> Result<Vec<usize>, ToggleSolutionError> {
        // Return the first solution of button push that returns the expected output
        let mut possible_solutions = VecDeque::new();
        possible_solutions.push_front((Vec::new(), vec![false; self.target_lights.len()]));
        let mut explored_positions = HashSet::new();

        // Security to make sure there is no infinite loop
        let limit: usize = 10_000_000;
        let mut i = 0;
        while i < limit {
            let (button_combination, lights_state) = possible_solutions
                .pop_front()
                .ok_or(ToggleSolutionError::EmptySolutionToExplore)?;
            let last_button = button_combination.last();
            for button in self.buttons.iter() {
                if let Some(last_button_index) = last_button {
                    if *last_button_index == button.index {
                        // Do not press back the last button pressed as it will simply revert to the previous state
                        continue;
                    }
                };
                let mut new_combination = button_combination.clone();
                new_combination.push(button.index);
                let mut new_state = lights_state.clone();
                button
                    .lights_activated
                    .iter()
                    .for_each(|&light_index| new_state[light_index] = !new_state[light_index]);
                if new_state == self.target_lights {
                    return Ok(new_combination);
                } else if !explored_positions.contains(&new_state) {
                    // Deduplicate positions that have already been seen
                    explored_positions.insert(new_state.clone());
                    possible_solutions.push_back((new_combination, new_state));
                };
            }
            i += 1;
        }
        Err(ToggleSolutionError::IterationLimitReached(i))
    }

    fn find_joltage_backtrack(&self) -> Result<usize, ToggleSolutionError> {
        println!("starting {:?}", &self.target_joltage);
        let target_state = vec![0; self.target_joltage.len()];
        let mut maximum_n_buttons = usize::MAX;
        let sorted_buttons = self.sort_buttons_by_joltage_counter_button_number();
        let start_time = SystemTime::now();

        let mut explored_states: HashMap<(usize, Vec<u16>), usize> = HashMap::new();

        fn backtrack(
            current_state: Vec<u16>,
            target_state: &Vec<u16>,
            current_n_buttons: usize,
            available_buttons: &Vec<Button>,
            current_button_index: usize,
            maximum_n_buttons: &mut usize,
            just_advanced_index: bool,
            explored_states: &mut HashMap<(usize, Vec<u16>), usize>,
        ) -> Result<Option<usize>, ToggleSolutionError> {
            dbg!(&current_n_buttons, &current_button_index, &current_state,);
            // Verify if current state is a success
            if &current_state == target_state {
                return Ok(Some(current_n_buttons));
            }
            // Else verify if it's a deadend
            if (current_button_index >= available_buttons.len())
            // No need to search a branch if we know it's longer than the current solution
                || (current_n_buttons >= *maximum_n_buttons)
                || if let Some(state_button_press) = explored_states.get(&(current_button_index, current_state.clone())) {
                    current_n_buttons >= *state_button_press
                } else {false}
            // If there are joltage counters that cannot be reached, then we get rid of it
            // Lazy evaluation only when starting to test this index
            || if just_advanced_index {!buttons_are_available_for_missing_joltage(
                &available_buttons[current_button_index..],
                &current_state,
            )} else {false}
            {
                return Ok(None);
            };
            // Memoize the search
            explored_states.insert(
                (current_button_index, current_state.clone()),
                current_n_buttons,
            );

            // If it's not a success nor a deadend, pursue the search
            let button = available_buttons
                .get(current_button_index)
                .expect("The button to be in the available buttons");
            let mut new_state = current_state.clone();
            for &joltage_index in button.lights_activated.iter() {
                if new_state[joltage_index] == 0 {
                    return Ok(None);
                } else {
                    new_state[joltage_index] -= 1
                }
            }
            if let Ok(Some(new_n_buttons)) = backtrack(
                new_state,
                target_state,
                current_n_buttons + 1,
                available_buttons,
                current_button_index,
                maximum_n_buttons,
                false,
                explored_states,
            ) {
                *maximum_n_buttons = min(new_n_buttons, *maximum_n_buttons);
            }
            if let Ok(Some(new_n_buttons)) = backtrack(
                current_state,
                target_state,
                current_n_buttons,
                available_buttons,
                current_button_index + 1,
                maximum_n_buttons,
                true,
                explored_states,
            ) {
                *maximum_n_buttons = min(new_n_buttons, *maximum_n_buttons)
            };
            if *maximum_n_buttons < usize::MAX {
                Ok(Some(*maximum_n_buttons))
            } else {
                Ok(None)
            }
        }
        let result = backtrack(
            self.target_joltage.clone(),
            &target_state,
            0,
            &sorted_buttons,
            0,
            &mut maximum_n_buttons,
            false,
            &mut explored_states,
        )?
        .ok_or(ToggleSolutionError::EmptySolutionToExplore);
        println!(
            "Took {:?}",
            (SystemTime::now().duration_since(start_time).unwrap())
        );
        result
    }

    fn sort_buttons_by_joltage_counter_button_number(&self) -> Vec<Button> {
        // Sort the buttons by the number of options a joltage counter has.
        // E.g if a button is the only one to trigger a joltage counter, it will be before
        // a button which triggers a joltage which has a second button which could be triggered.

        let mut final_buttons = Vec::with_capacity(self.buttons.len());
        let mut joltages = (0..self.target_joltage.len()).collect::<Vec<usize>>();
        let mut buttons = self.buttons.clone();
        while !joltages.is_empty() & !buttons.is_empty() {
            let button_counts_per_joltage = count_joltage_counter_number_of_buttons(&buttons);
            // Sort in descending order
            joltages.sort_by(|joltage_a, joltage_b| {
                button_counts_per_joltage
                    .get(joltage_b)
                    .cmp(&button_counts_per_joltage.get(joltage_a))
                    .then(self.target_joltage[*joltage_b].cmp(&self.target_joltage[*joltage_a]))
            });
            // Take last element
            let joltage_with_most_priority =
                joltages.pop().expect("The joltage to not be empty yet");
            buttons
                .extract_if(.., |button| {
                    button
                        .lights_activated
                        .contains(&joltage_with_most_priority)
                })
                .sorted_by_key(|button| button.lights_activated.len())
                .rev()
                .for_each(|button| final_buttons.push(button));
        }

        final_buttons
    }
}

fn count_joltage_counter_number_of_buttons(buttons: &Vec<Button>) -> HashMap<&usize, i32> {
    let mut button_counts_per_joltage = HashMap::new();
    buttons
        .iter()
        .map(
            |Button {
                 lights_activated, ..
             }| lights_activated,
        )
        .flatten()
        .for_each(|joltage_index| {
            button_counts_per_joltage.insert(
                joltage_index,
                button_counts_per_joltage
                    .get(joltage_index)
                    .or(Some(&0))
                    .expect("The value to be present or inserted")
                    + 1,
            );
        });
    button_counts_per_joltage
}

fn buttons_are_available_for_missing_joltage(
    remaining_buttons: &[Button],
    current_state: &Vec<u16>,
) -> bool {
    // Check the remaining buttons can increase any joltage that still requires it
    let remaining_joltage_indexes = current_state
        .iter()
        .enumerate()
        .filter_map(
            |(i, target_joltage)| {
                if target_joltage > &0 { Some(i) } else { None }
            },
        )
        .collect::<HashSet<usize>>();
    let remaining_push_buttons = remaining_buttons
        .iter()
        .map(|button| button.lights_activated.clone())
        .flatten()
        .collect::<HashSet<usize>>();
    remaining_push_buttons
        .intersection(&remaining_joltage_indexes)
        .copied()
        .collect::<HashSet<usize>>()
        == remaining_joltage_indexes
}

#[derive(Error, Debug)]
enum ToggleSolutionError {
    #[error("Maximum iteration of {0} reached")]
    IterationLimitReached(usize),
    #[error("No more solutions to try")]
    EmptySolutionToExplore,
    // #[error("Got above the maximum joltage")]
    // GotAboveMaximumJoltage,
}

fn parse_input(input: String) -> Result<Vec<Machine>> {
    input
        .lines()
        .map(Machine::new)
        .collect::<Result<Vec<Machine>>>()
}

// TODO implement width first search with stopping as soon as we hit the target lights
pub fn main() {
    let input = parse_input(read_to_string("data/day_10.txt").expect("The data to be readable"))
        .expect("The input to be correctly parsed");
    // First problem solution
    let first_problem_solution: usize = input
        .iter()
        .map(|machine| {
            machine
                .find_fastest_way_to_toogle_light_panel()
                .expect("The lights combination to have a solution")
                .len()
        })
        .sum();
    println!(
        "The minimal number of button presses to get to the correct lights is {}",
        first_problem_solution
    );
    let second_problem_solution: usize = input
        .into_iter()
        .parallel_map(|machine| {
            machine
                .find_joltage_backtrack()
                .expect("A solution to exist to find the correct joltage")
        })
        .sum();
    println!(
        "The minimal number of presses to get to the correct joltage is {}",
        second_problem_solution
    );
}
