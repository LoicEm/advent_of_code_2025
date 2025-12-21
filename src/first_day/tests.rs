use crate::first_day::{Dial, RotationInstruction};

#[test]
fn test_dial_going_left_goes_back_to_correct_value() {
    let mut dial = Dial::new(0);
    let instruction = RotationInstruction {
        direction: super::Direction::Left,
        distance: 1,
    };
    dial.rotate(instruction);
    assert_eq!(dial.position, 99);
    assert_eq!(dial.n_times_passed_on_zero, 0);
}

#[test]
fn test_dial_going_more_than_one_turn_left_has_correct_number_of_passes_on_zero() {
    let mut dial = Dial::new(0);
    let instruction = RotationInstruction {
        direction: super::Direction::Left,
        distance: 501,
    };
    dial.rotate(instruction);
    assert_eq!(dial.position, 99);
    assert_eq!(dial.n_times_passed_on_zero, 5);
}

#[test]
fn test_dial_going_right_goes_back_to_valid_value() {
    let mut dial = Dial::new(0);
    let instruction = RotationInstruction {
        direction: super::Direction::Right,
        distance: 110,
    };
    dial.rotate(instruction);
    assert_eq!(dial.position, 10);
    assert_eq!(dial.n_times_passed_on_zero, 1);
}

#[test]
fn test_dial_going_more_than_one_turn_right_has_correct_number_of_passes_on_zero() {
    let mut dial = Dial::new(0);
    let instruction = RotationInstruction {
        direction: super::Direction::Right,
        distance: 210,
    };
    dial.rotate(instruction);
    assert_eq!(dial.position, 10);
    assert_eq!(dial.n_times_passed_on_zero, 2);
}

#[test]
fn test_dial_going_small_right_is_ok() {
    let mut dial = Dial::new(90);
    let instruction = RotationInstruction {
        direction: super::Direction::Right,
        distance: 9,
    };
    dial.rotate(instruction);
    assert_eq!(dial.position, 99);
    assert_eq!(dial.n_times_passed_on_zero, 0);
}

#[test]
fn test_dial_going_small_left_is_ok() {
    let mut dial = Dial::new(90);
    let instruction = RotationInstruction {
        direction: super::Direction::Left,
        distance: 9,
    };
    dial.rotate(instruction);
    assert_eq!(dial.position, 81);
    assert_eq!(dial.n_times_passed_on_zero, 0);
}

#[test]
fn test_dial_stopping_on_zero_then_going_left_is_counted_only_one_pass_on_zero() {
    let mut dial = Dial::new(1);
    let first_instruction = RotationInstruction {
        direction: super::Direction::Right,
        distance: 99,
    };
    let second_instruction = RotationInstruction {
        direction: super::Direction::Left,
        distance: 2,
    };
    dial.rotate(first_instruction);
    assert_eq!(dial.position, 0);
    assert_eq!(dial.n_times_passed_on_zero, 1);
    dial.rotate(second_instruction);
    assert_eq!(dial.position, 98);
    assert_eq!(dial.n_times_passed_on_zero, 1);
}
