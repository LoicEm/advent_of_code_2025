use std::collections::HashSet;

use crate::seventh_day::{
    QuanticTachyonBeams, QuanticTachyonManifoldResult, TachyonBeams, TachyonManifold,
};

#[test]
fn test_tachyonbeam_advance() {
    let beam = TachyonBeams {
        current_position: HashSet::from([1, 3, 8]),
        n_splits: 0,
    };
    let splitter_line: HashSet<usize> = [1, 3].into();
    assert_eq!(
        beam.advance(&splitter_line),
        TachyonBeams {
            current_position: HashSet::from([0, 2, 4, 8]),
            n_splits: 2
        }
    );
}

#[test]
fn test_split_quantic_beam() {
    let beam = QuanticTachyonBeams {
        current_position: HashSet::from([1, 3, 8]),
        n_previous_timelines: 2,
    };
    let mut expected_output = vec![
        QuanticTachyonBeams {
            current_position: HashSet::from([0, 3, 8]),
            n_previous_timelines: 2,
        },
        QuanticTachyonBeams {
            current_position: HashSet::from([2, 3, 8]),
            n_previous_timelines: 2,
        },
    ];
    let mut output = beam.split(1);
    expected_output.sort();
    output.sort();
    assert_eq!(output, expected_output)
}

#[test]
fn test_quantic_tachyonbeam_advance() {
    // Might fail because of ordering issue
    let beam = QuanticTachyonBeams {
        current_position: HashSet::from([1, 3, 8]),
        n_previous_timelines: 2,
    };
    let splitter_line: HashSet<usize> = [1, 3].into();
    let mut expected_output = vec![
        QuanticTachyonBeams {
            current_position: HashSet::from([0, 2, 8]),
            n_previous_timelines: 2,
        },
        QuanticTachyonBeams {
            current_position: HashSet::from([0, 4, 8]),
            n_previous_timelines: 2,
        },
        QuanticTachyonBeams {
            current_position: HashSet::from([2, 8]),
            n_previous_timelines: 2,
        },
        QuanticTachyonBeams {
            current_position: HashSet::from([2, 4, 8]),
            n_previous_timelines: 2,
        },
    ];
    let mut output = beam.advance(&splitter_line);
    expected_output.sort();
    output.sort();
    assert_eq!(output, expected_output)
}

#[test]
fn test_quantic_manifold_run() {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
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
    let result = manifold.run_quantic();
    assert_eq!(result, QuanticTachyonManifoldResult { n_timelines: 40 })
}
