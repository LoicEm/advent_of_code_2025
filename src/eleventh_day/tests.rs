use std::collections::HashSet;

use crate::eleventh_day::{
    PathFindingStatus, find_n_paths_with_2_intermediate_steps, find_paths, parse_input,
    topological_sort,
};

static INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

#[test]
fn test_first_problem_example() {
    let devices = parse_input(INPUT.to_string());
    let result = find_paths(
        &devices,
        "you",
        "out",
        HashSet::new(),
        &HashSet::new(),
        // &mut HashSet::new(),
    )
    .expect("The function to work");
    match result {
        PathFindingStatus::ValidPathFound(paths) => assert_eq!(paths.iter().count(), 5),
        _ => panic!("A valid path should be found"),
    };
}

static INPUT_SECOND_PROBLEM: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

#[test]
fn test_second_problem() {
    let devices = parse_input(INPUT_SECOND_PROBLEM.to_string());
    let topologically_sorted_nodes =
        topological_sort(&devices).expect("The nodes to be topologically sorted");
    let result = find_n_paths_with_2_intermediate_steps(
        &topologically_sorted_nodes,
        "svr",
        "out",
        "fft",
        "dac",
    );
    assert_eq!(result, 2)
}
