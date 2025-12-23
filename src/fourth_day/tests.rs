use crate::fourth_day::RollOfPaper;

#[test]
fn test_same_roll_is_not_adjacent() {
    let roll = RollOfPaper { x: 1, y: 1 };
    assert!(!roll.is_adjacent(&roll))
}

#[test]
fn test_adjacent_roll_just_above() {
    let roll = RollOfPaper { x: 1, y: 1 };
    let other_roll = RollOfPaper { x: 1, y: 2 };
    assert!(roll.is_adjacent(&other_roll))
}

#[test]
fn test_adjacent_roll_just_below() {
    let roll = RollOfPaper { x: 1, y: 1 };
    let other_roll = RollOfPaper { x: 1, y: 0 };
    assert!(roll.is_adjacent(&other_roll))
}

#[test]
fn test_adjacent_roll_just_left() {
    let roll = RollOfPaper { x: 1, y: 1 };
    let other_roll = RollOfPaper { x: 0, y: 1 };
    assert!(roll.is_adjacent(&other_roll))
}

#[test]
fn test_adjacent_roll_just_right() {
    let roll = RollOfPaper { x: 1, y: 1 };
    let other_roll = RollOfPaper { x: 2, y: 1 };
    assert!(roll.is_adjacent(&other_roll))
}

#[test]
fn test_adjacent_roll_just_diagonal() {
    let roll = RollOfPaper { x: 1, y: 1 };
    let other_roll = RollOfPaper { x: 2, y: 2 };
    assert!(roll.is_adjacent(&other_roll))
}

#[test]
fn test_non_adjacent_roll() {
    let roll = RollOfPaper { x: 1, y: 1 };
    let other_roll = RollOfPaper { x: 3, y: 3 };
    assert!(!roll.is_adjacent(&other_roll))
}

#[test]
fn test_non_adjacent_roll_same_column() {
    let roll = RollOfPaper { x: 1, y: 1 };
    let other_roll = RollOfPaper { x: 3, y: 1 };
    assert!(!roll.is_adjacent(&other_roll))
}

#[test]
fn test_non_adjacent_roll_same_row() {
    let roll = RollOfPaper { x: 1, y: 1 };
    let other_roll = RollOfPaper { x: 1, y: 3 };
    assert!(!roll.is_adjacent(&other_roll))
}

#[test]
fn test_non_adjacent_roll_close_row() {
    let roll = RollOfPaper { x: 1, y: 1 };
    let other_roll = RollOfPaper { x: 2, y: 3 };
    assert!(!roll.is_adjacent(&other_roll))
}

#[test]
fn test_non_adjacent_roll_close_column() {
    let roll = RollOfPaper { x: 1, y: 1 };
    let other_roll = RollOfPaper { x: 4, y: 2 };
    assert!(!roll.is_adjacent(&other_roll))
}
