use std::{
    collections::HashSet,
    fs::read_to_string
};
use itertools::Itertools;
use test_case::test_case;
use crate::ninth_day::{
    FigureIntervals, GreenInterval, LineGreenIntervals, RedTile, calculate_area, get_tile_combinations, parse_input
};

static INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

#[test]
fn test_example_first_case() {
    let input = parse_input(INPUT.to_string());
    let biggest_line = get_tile_combinations(&input)
        .into_iter()
        .map(|(a, b)| calculate_area(&a, &b))
        .max()
        .expect("That there are some possible");
    assert_eq!(biggest_line, 50)
}

#[test]
fn input_data_does_not_have_three_consecutive_red_tiles_aligned() {
    // Test that the input data does not have three consecutives red tiles aligned
    // If that is the case, this would simplify the way we can draw the green tile area
    let input =
        parse_input(read_to_string("data/day_9.txt").expect("The data file to be correctly read."));
    let mut input_iter = input.into_iter();
    let mut previous_tile = input_iter
        .next()
        .expect("The input to have at least a red tile");
    let mut current_tile = input_iter
        .next()
        .expect("The input to have at least two red tiles");
    for next_tile in input_iter {
        if current_tile.x == next_tile.x {
            assert_ne!(previous_tile.x, next_tile.x)
        } else if current_tile.y == next_tile.y {
            assert_ne!(previous_tile.y, next_tile.y)
        } else {
            panic!("Coordinates X or Y should be the same on consecutives red tiles")
        };
        previous_tile = current_tile;
        current_tile = next_tile;
    }
}

fn create_tiles(x_s: Vec<usize>, y: usize) -> Vec<RedTile> {
    x_s.into_iter().map(|x| RedTile { x, y }).collect()
}

#[test]
fn test_first_line_interval_creation() {
    // For the first line, the previous line is empty
    let tiles = create_tiles(vec![1, 2, 5, 7], 0);
    let line =
        LineGreenIntervals::new_first_line(&tiles).expect("The line to be correctly created");
    assert!(
        line.current_intervals
            == HashSet::from([
                GreenInterval { start: 1, end: 2 },
                GreenInterval { start: 5, end: 7 }
            ])
    )
}

#[test]
fn test_line_creation_merge_intervals() {
    let previous_line = LineGreenIntervals {
        // Current intervals is not used in this test
        current_intervals: HashSet::new(),
        next_intervals: HashSet::from([
            GreenInterval { start: 1, end: 3 },
            GreenInterval { start: 5, end: 7 },
        ]),
    };
    let tiles = create_tiles(vec![3, 5], 10);
    let line =
        LineGreenIntervals::new(&tiles, previous_line).expect("The line to be correctly created");
    assert_eq!(
        line.current_intervals,
        HashSet::from([GreenInterval { start: 1, end: 7 }]), "current intervals do not match"
    );
    assert_eq!(
        line.next_intervals,
        HashSet::from([GreenInterval { start: 1, end: 7 }]), "next intervals do not match"
    )
}

#[test]
fn test_line_creation_split_interval() {
    let previous_line = LineGreenIntervals {
        current_intervals: HashSet::from([GreenInterval { start: 1, end: 9 }]),
        next_intervals: HashSet::from([GreenInterval { start: 1, end: 9 }]),
    };
    let tiles = create_tiles(vec![3, 5], 2);
    let line =
        LineGreenIntervals::new(&tiles, previous_line).expect("The line to be correctly created");
    assert_eq!(
        line.current_intervals,
        HashSet::from([GreenInterval { start: 1, end: 9 }])
    );
    assert_eq!(
        line.next_intervals,
        HashSet::from([
            GreenInterval { start: 1, end: 3 },
            GreenInterval { start: 5, end: 9 }
        ])
    )
}

#[test_case(create_tiles(vec![1, 3], 2), GreenInterval {start: 3, end: 10};"from the right")]
#[test_case(create_tiles(vec![7, 10], 2), GreenInterval { start: 1, end: 7 };"from the left")]
#[test_case(create_tiles(vec![1, 3, 7, 10], 2), GreenInterval { start: 3, end: 7 };"from both sides")]
fn test_reduce_line(tiles: Vec<RedTile>, expected_next_interval: GreenInterval) {
    let previous_line = LineGreenIntervals {
        current_intervals: HashSet::from([GreenInterval { start: 1, end: 10 }]),
        next_intervals: HashSet::from([GreenInterval { start: 1, end: 10 }]),
    };
    let line =
        LineGreenIntervals::new(&tiles, previous_line).expect("The line to be correctly created");
    assert_eq!(
        line.current_intervals,
        HashSet::from([GreenInterval { start: 1, end: 10 }])
    );
    assert_eq!(line.next_intervals, HashSet::from([expected_next_interval]))
}

#[test_case(create_tiles(vec![1, 3], 2), GreenInterval {start: 1, end: 7};"to the left")]
#[test_case(create_tiles(vec![7, 10], 2), GreenInterval { start: 3, end: 10 };"to the right")]
#[test_case(create_tiles(vec![1, 3, 7, 10], 2), GreenInterval { start: 1, end: 10 };"to both sides")]
fn test_extend_line(tiles: Vec<RedTile>, expected_next_interval: GreenInterval) {
    let previous_line = LineGreenIntervals {
        current_intervals: HashSet::from([GreenInterval { start: 3, end: 7 }]),
        next_intervals: HashSet::from([GreenInterval { start: 3, end: 7 }]),
    };
    let line =
        LineGreenIntervals::new(&tiles, previous_line).expect("The line to be correctly created");
    assert_eq!(
        line.current_intervals,
        HashSet::from([expected_next_interval.clone()])
    );
    assert_eq!(
        line.next_intervals,
        HashSet::from([expected_next_interval.clone()])
    )
}

#[test_case(
    create_tiles(vec![1, 3, 5, 7], 3), 
    GreenInterval {start: 1, end: 7}, 
    GreenInterval {start: 1, end: 5}; 
    "To the left"
)]
#[test_case(
    create_tiles(vec![3, 5, 7, 9], 3), 
    GreenInterval {start: 3, end: 9}, 
    GreenInterval {start: 5, end: 9}; 
    "To the right"
)]
fn test_move_line(tiles: Vec<RedTile>, expected_current_interval: GreenInterval, expected_next_interval: GreenInterval) {
    let previous_intervals = HashSet::from([GreenInterval { start: 3, end: 7 }]);
    let previous_line = LineGreenIntervals {
        current_intervals: previous_intervals.clone(),
        next_intervals: previous_intervals.clone(),
    };
    let line =
        LineGreenIntervals::new(&tiles, previous_line).expect("The line to be correctly computed");

    assert_eq!(
        line.current_intervals,
        HashSet::from([expected_current_interval.clone()])
    );

    assert_eq!(
        line.next_intervals,
        HashSet::from([expected_next_interval.clone()])
    );
}

#[test_case(create_tiles(
    vec![3, 5, 7, 9], 2), 
    HashSet::from([
        GreenInterval{start: 1, end: 3}, 
        GreenInterval{start: 5, end: 7},
        GreenInterval{start: 9, end: 15}
    ]);
    "two splits"
)]
#[test_case(create_tiles(
    vec![3, 5, 7, 9, 12, 14], 2), 
    HashSet::from([
        GreenInterval{start: 1, end: 3}, 
        GreenInterval{start: 5, end: 7},
        GreenInterval{start: 9, end: 12}, 
        GreenInterval{start: 14, end: 15}, 
    ]); 
    "three splits"
)]
fn test_multiple_splits_on_same_line(tiles: Vec<RedTile>, expected_result: HashSet<GreenInterval>) {
    let current_intervals = HashSet::from([GreenInterval { start: 1, end: 15 }]);
    let previous_line = LineGreenIntervals {
        current_intervals: current_intervals.clone(),
        next_intervals: current_intervals.clone(),
    };
    let line =
        LineGreenIntervals::new(&tiles, previous_line).expect("The line to be correctly created");
    assert_eq!(
        line.current_intervals,
        current_intervals.clone());
    assert_eq!(
        line.next_intervals,
        expected_result
    )
}

#[test_case(
    create_tiles(vec![3, 6, 8, 12], 3), 
    HashSet::from([
        GreenInterval {start: 1, end: 3}, 
        GreenInterval {start: 6, end: 8}, 
        GreenInterval {start: 12, end: 20}
        ]); 
    "two merges")]
#[test_case(create_tiles(vec![3, 6, 8, 10, 12,16], 3), 
    HashSet::from([
        GreenInterval {start: 1, end: 3},
        GreenInterval {start: 6, end: 8},
        GreenInterval {start: 10, end: 12},
        GreenInterval {start: 16, end: 20}
    ]); 
    "three merges")]
fn test_multi_merge_on_same_line(tiles: Vec<RedTile>, current_intervals: HashSet<GreenInterval>) {
    let previous_line = LineGreenIntervals {
        current_intervals: current_intervals.clone(),
        next_intervals: current_intervals.clone()
    };
    let line = LineGreenIntervals::new(&tiles, previous_line).expect("The line to be correctly created");
    let expected_result = HashSet::from([GreenInterval {start: 1, end: 20}]);
    assert_eq!(line.current_intervals, expected_result);
    assert_eq!(line.next_intervals, expected_result);
}

#[test]
fn test_example_second_case() {
    let input = parse_input(INPUT.to_string());
    let figure = FigureIntervals::new(&input).expect("The figure to be correctly created");
    dbg!(&figure.lines);
    let biggest_rectangle = get_tile_combinations(&input)
        .iter()
        .sorted_by_key(|(tile_a, tile_b)| calculate_area(tile_a, tile_b))
        .rev()
        .find(|(tile_a, tile_b)| figure.rectangle_is_within_figure(tile_a, tile_b))
        .expect("There is at least one valid rectangle")
        .clone();
    let (tile_a, tile_b) = biggest_rectangle;
    assert_eq!(calculate_area(&tile_a, &tile_b), 24)
}


