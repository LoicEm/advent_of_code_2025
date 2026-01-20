fn find_joltage_ida_star(&self) -> Result<usize, ToggleSolutionError> {
    println!("Starting {:?}", &self.target_joltage);
    let start_time = SystemTime::now();

    let target = vec![0; self.target_joltage.len()];
    let state = self.target_joltage.clone();
    let mut transposition = HashMap::new();
    let mut visited_states = HashSet::new();

    fn heuristic(state: &Vec<usize>, max_button_power: usize) -> usize {
        *state.iter().max().unwrap_or(&0)
    }

    fn apply(button: &Button, state: &Vec<usize>) -> Option<Vec<usize>> {
        let mut new_state = state.clone();
        for counter_index in button.lights_activated.iter() {
            if state[*counter_index] == 0 {
                return None;
            } else {
                new_state[*counter_index] -= 1
            }
        }
        Some(new_state)
    }

    enum DepthFirstSearchResult {
        Success(usize),
        Failure(usize),
    }

    fn depth_first_search(
        state: &Vec<usize>,
        current_cost: usize,
        bound_cost: usize,
        available_buttons: &Vec<Button>,
        target_joltage: &Vec<usize>,
        max_button_power: usize,
        transposition: &mut HashMap<Vec<usize>, usize>,
        visited_states: &mut HashSet<Vec<usize>>,
    ) -> DepthFirstSearchResult {
        let heuristic_value = heuristic(state, max_button_power);
        let estimated_cost = current_cost + heuristic_value;
        if estimated_cost > bound_cost {
            return DepthFirstSearchResult::Failure(estimated_cost);
        } else if state == target_joltage {
            return DepthFirstSearchResult::Success(current_cost);
        } else {
            if let Some(&best_cost) = transposition.get(state) {
                if current_cost >= best_cost {
                    return DepthFirstSearchResult::Failure(bound_cost);
                }
            }
            transposition.insert(state.clone(), current_cost);
            visited_states.insert(state.clone());
            let mut min_next_bound = usize::MAX;
            for button in available_buttons {
                if let Some(next_state) = apply(button, state) {
                    if visited_states.contains(&next_state) {
                        continue;
                    }
                    match depth_first_search(
                        &next_state,
                        current_cost + 1,
                        bound_cost,
                        available_buttons,
                        target_joltage,
                        max_button_power,
                        transposition,
                        visited_states,
                    ) {
                        DepthFirstSearchResult::Success(cost) => {
                            return DepthFirstSearchResult::Success(cost);
                        }
                        DepthFirstSearchResult::Failure(next_bound) => {
                            min_next_bound = min_next_bound.min(next_bound);
                        }
                    }
                }
            }
            visited_states.remove(state);
            DepthFirstSearchResult::Failure(min_next_bound)
        }
    }

    let max_button_power = self
        .buttons
        .iter()
        .map(
            |Button {
                 lights_activated, ..
             }| lights_activated.len(),
        )
        .max()
        .expect("To have at least one button");

    let mut bound_cost = heuristic(&state, max_button_power);
    loop {
        visited_states.clear();
        match depth_first_search(
            &state,
            0,
            bound_cost,
            &self.buttons,
            &target,
            max_button_power,
            &mut transposition,
            &mut visited_states,
        ) {
            DepthFirstSearchResult::Success(cost) => {
                println!(
                    "Took {:?}",
                    SystemTime::now().duration_since(start_time).unwrap()
                );
                return Ok(cost);
            }
            DepthFirstSearchResult::Failure(next_bound) => {
                if next_bound == usize::MAX {
                    return Err(ToggleSolutionError::EmptySolutionToExplore);
                } else {
                    bound_cost = next_bound
                }
            }
        }
    }
}
