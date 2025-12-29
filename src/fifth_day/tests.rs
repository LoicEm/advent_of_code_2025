use crate::fifth_day::{ProcessingIdRange, count_number_of_fresh_ingredients};

#[test]
fn test_fresh_ingredients() {
    let ranges = vec![
        ProcessingIdRange { start: 1, end: 3 },
        ProcessingIdRange { start: 10, end: 11 },
    ];
    assert_eq!(count_number_of_fresh_ingredients(&ranges), 5)
}

#[test]
fn test_overlapping_ranges() {
    let ranges = vec![
        ProcessingIdRange { start: 1, end: 3 },
        ProcessingIdRange { start: 3, end: 6 },
    ];
    assert_eq!(count_number_of_fresh_ingredients(&ranges), 6)
}

#[test]
fn test_contained_ranges() {
    let ranges = vec![
        ProcessingIdRange { start: 1, end: 6 },
        ProcessingIdRange { start: 3, end: 5 },
    ];
    assert_eq!(count_number_of_fresh_ingredients(&ranges), 6)
}

#[test]
fn test_overlapping_with_one_more_element_ranges() {
    let ranges = vec![
        ProcessingIdRange { start: 1, end: 6 },
        ProcessingIdRange { start: 6, end: 7 },
    ];
    assert_eq!(count_number_of_fresh_ingredients(&ranges), 7)
}

#[test]
fn test_contained_range_ending_on_same_value() {
    let ranges = vec![
        ProcessingIdRange { start: 1, end: 6 },
        ProcessingIdRange { start: 5, end: 6 },
    ];
    assert_eq!(count_number_of_fresh_ingredients(&ranges), 6)
}
