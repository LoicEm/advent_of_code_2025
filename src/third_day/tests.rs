use crate::third_day::Battery;

#[test]
fn test_battery_ordering() {
    let first_battery = Battery {
        position: 0,
        joltage: 2,
    };
    let second_battery = Battery {
        position: 1,
        joltage: 3,
    };
    assert!(first_battery < second_battery)
}

#[test]
fn test_first_battery_ordering() {
    let first_battery = Battery {
        position: 0,
        joltage: 4,
    };
    let second_battery = Battery {
        position: 1,
        joltage: 3,
    };
    assert!(first_battery > second_battery)
}

#[test]
fn test_battery_with_same_joltage_first_one_is_greater() {
    let first_battery = Battery {
        position: 0,
        joltage: 3,
    };
    let second_battery = Battery {
        position: 1,
        joltage: 3,
    };
    assert!(first_battery > second_battery)
}
