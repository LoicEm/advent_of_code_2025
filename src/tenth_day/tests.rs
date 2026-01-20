use std::collections::HashSet;

use crate::tenth_day::{Button, Machine, parse_input};

static INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

#[test]
fn test_first_problem_example() {
    let input = parse_input(INPUT.to_string()).expect("The input to be correctly parsed");
    let first_problem_solution: usize = input
        .iter()
        .map(|machine| {
            machine
                .find_fastest_way_to_toogle_light_panel()
                .expect("The lights combination to have a solution")
                .len()
        })
        .sum();
    assert_eq!(first_problem_solution, 7)
}

#[test]
fn test_second_problem_example() {
    let input = parse_input(INPUT.to_string()).expect("The input to be correctly parsed");
    let second_problem_solution: usize = input
        .iter()
        .map(|machine| {
            machine
                .find_joltage_backtrack()
                .expect("The joltage combination to have a solution")
        })
        .sum();
    assert_eq!(second_problem_solution, 33)
}

#[test]
fn test_button_ordering() {
    let machine = Machine::new("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
        .expect("The machine to be correctly created");
    let sorted_buttons = machine.sort_buttons_by_joltage_counter_button_number();
    assert_eq!(
        sorted_buttons,
        vec![
            Button {
                index: 5,
                lights_activated: HashSet::from([0, 1])
            },
            Button {
                index: 4,
                lights_activated: HashSet::from([0, 2])
            },
            Button {
                index: 1,
                lights_activated: HashSet::from([1, 3])
            },
            Button {
                index: 3,
                lights_activated: HashSet::from([2, 3])
            },
            Button {
                index: 2,
                lights_activated: HashSet::from([2])
            },
            Button {
                index: 0,
                lights_activated: HashSet::from([3])
            },
        ]
    )
}
