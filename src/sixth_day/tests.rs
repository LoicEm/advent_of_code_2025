use crate::sixth_day::{get_split_columns_index, split_problems_line};

#[test]
fn test_get_split_index() {
    let output = get_split_columns_index(&"+  *   + *".to_string());
    assert_eq!(output, vec![2, 6, 8])
}

#[test]
fn test_split_by_index() {
    let split_indexes = vec![2, 7, 9];
    let line = "12 134  2 23  ";
    let output = split_problems_line(line, &split_indexes);
    assert_eq!(output, vec!["12", "134 ", "2", "23  "])
}
