use crate::second_day::number_is_silly_part_two;

#[test]
fn test_part2_long_silly_number() {
    assert!(number_is_silly_part_two(1212121212))
}

#[test]
fn test_part2_long_unsilly_number() {
    assert!(!number_is_silly_part_two(1231231231))
}

#[test]
fn test_part2_repeated_number() {
    assert!(number_is_silly_part_two(11111111111))
}

#[test]
fn test_part2_length_9_silly_number() {
    assert!(number_is_silly_part_two(123123123))
}

#[test]
fn test_part2_length_9_unsilly_number() {
    assert!(!number_is_silly_part_two(123123124))
}

#[test]
fn test_part2_length_10_silly_number() {
    assert!(number_is_silly_part_two(1212121212))
}

#[test]
fn test_part2_length_6_silly_number() {
    assert!(number_is_silly_part_two(121212))
}
