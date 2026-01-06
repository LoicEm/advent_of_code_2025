use itertools::{Itertools, iproduct};
use std::collections::{HashMap, HashSet};

use crate::eight_day::{
    Circuit, CircuitMapping, DistanceMapping, JunctionBoxCoordinates, parse_input,
};

#[test]
fn test_box_distance() {
    let box_a = JunctionBoxCoordinates { x: 1, y: 0, z: 0 };
    let box_b = JunctionBoxCoordinates { x: 1, y: 0, z: 1 };
    let box_c = JunctionBoxCoordinates { x: 1, y: 1, z: 1 };
    assert_eq!(box_a.distance(&box_b), 1);
    assert_eq!(box_b.distance(&box_a), 1);
    assert_eq!(box_a.distance(&box_c), 2);
}

#[test]
fn test_merge_circuits() {
    let mut circuit_mapping = CircuitMapping {
        junction_box_id_to_circuit: HashMap::from([
            (0, Circuit { id: 0 }),
            (1, Circuit { id: 2 }),
            (2, Circuit { id: 2 }),
            (3, Circuit { id: 3 }),
        ]),
        junction_box_id_to_junction_box: HashMap::new(),
    };
    circuit_mapping.merge_circuits(2, 0);
    assert_eq!(
        circuit_mapping
            .get_circuit_boxes(2)
            .into_iter()
            .collect::<HashSet<usize>>(),
        HashSet::from([0, 1, 2])
    );
}

#[test]
fn test_merge_circuits_multiple_updates() {
    let mut circuit_mapping = CircuitMapping {
        junction_box_id_to_circuit: HashMap::from([
            (0, Circuit { id: 0 }),
            (1, Circuit { id: 2 }),
            (2, Circuit { id: 2 }),
            (3, Circuit { id: 3 }),
        ]),
        junction_box_id_to_junction_box: HashMap::new(),
    };
    circuit_mapping.merge_circuits(0, 2);
    assert_eq!(
        circuit_mapping
            .get_circuit_boxes(0)
            .into_iter()
            .collect::<HashSet<usize>>(),
        HashSet::from([0, 1, 2])
    );
}

#[test]
fn test_build_connections() {
    let mut circuit_mapping = CircuitMapping {
        junction_box_id_to_circuit: HashMap::from([
            (0, Circuit { id: 0 }),
            (1, Circuit { id: 1 }),
            (2, Circuit { id: 2 }),
            (3, Circuit { id: 3 }),
            (4, Circuit { id: 4 }),
        ]),
        junction_box_id_to_junction_box: HashMap::new(),
    };
    let distance_mapping = DistanceMapping {
        mapping: HashMap::from([
            ((0, 1), 10),
            ((0, 2), 1),
            ((0, 3), 5),
            ((1, 2), 2),
            ((3, 4), 3),
        ]),
    };
    circuit_mapping.build_connections(&distance_mapping, Some(3));
    assert_eq!(
        circuit_mapping
            .get_circuit_boxes(1)
            .into_iter()
            .collect::<HashSet<usize>>(),
        HashSet::from([0, 1, 2])
    );
    assert_eq!(
        circuit_mapping
            .get_circuit_boxes(3)
            .into_iter()
            .collect::<HashSet<usize>>(),
        HashSet::from([3, 4])
    );
    assert!(circuit_mapping.get_circuit_boxes(0).is_empty());
    assert!(circuit_mapping.get_circuit_boxes(2).is_empty());
    assert!(circuit_mapping.get_circuit_boxes(4).is_empty());
}

#[test]
fn test_boxes_part_of_the_same_circuit_are_not_connected_again() {
    let mut circuit_mapping = CircuitMapping {
        junction_box_id_to_circuit: HashMap::from([
            (0, Circuit { id: 0 }),
            (1, Circuit { id: 1 }),
            (2, Circuit { id: 2 }),
            (3, Circuit { id: 3 }),
            (4, Circuit { id: 4 }),
        ]),
        junction_box_id_to_junction_box: HashMap::new(),
    };
    let distance_mapping = DistanceMapping {
        mapping: HashMap::from([
            ((0, 1), 10),
            ((0, 2), 1),
            ((0, 3), 5),
            ((1, 2), 2),
            ((3, 4), 3),
        ]),
    };
    circuit_mapping.build_connections(&distance_mapping, Some(3));
    assert_eq!(
        circuit_mapping
            .get_circuit_boxes(1)
            .into_iter()
            .collect::<HashSet<usize>>(),
        HashSet::from([0, 1, 2])
    );
    assert_eq!(
        circuit_mapping
            .get_circuit_boxes(3)
            .into_iter()
            .collect::<HashSet<usize>>(),
        HashSet::from([3, 4])
    );
    assert!(circuit_mapping.get_circuit_boxes(0).is_empty());
    assert!(circuit_mapping.get_circuit_boxes(2).is_empty());
    assert!(circuit_mapping.get_circuit_boxes(4).is_empty());
}

static INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

#[test]
fn test_example_first_case() {
    let input = parse_input(INPUT.to_string());
    let mut circuit_mapping = CircuitMapping::new(&input);
    let mut distances = DistanceMapping {
        mapping: HashMap::new(),
    };
    // Since setting distance is bidirectional, we can set it on only half the junction boxes
    iproduct!(input[..input.len()].iter(), input.iter()).for_each(|(box_a, box_b)| {
        if box_a.index != box_b.index {
            distances.set(
                box_a.index,
                box_b.index,
                box_a.coordinates.distance(&box_b.coordinates),
            )
        }
    });

    circuit_mapping.build_connections(&distances, Some(10));

    let result: usize = circuit_mapping
        .get_circuits_size()
        .values()
        .sorted()
        .rev()
        .take(3)
        .map(|elem| {
            dbg!(elem);
            elem
        })
        .product();
    assert_eq!(result, 40);
}

#[test]
fn test_example_second_case() {
    let input = parse_input(INPUT.to_string());
    let mut circuit_mapping = CircuitMapping::new(&input);
    let mut distances = DistanceMapping {
        mapping: HashMap::new(),
    };
    // Since setting distance is bidirectional, we can set it on only half the junction boxes
    iproduct!(input[..input.len()].iter(), input.iter()).for_each(|(box_a, box_b)| {
        if box_a.index != box_b.index {
            distances.set(
                box_a.index,
                box_b.index,
                box_a.coordinates.distance(&box_b.coordinates),
            )
        }
    });

    let (last_box_a, last_box_b) = circuit_mapping
        .build_connections(&distances, None)
        .expect("A single circuit to eventually connect all boxes");
    assert_eq!(last_box_a.coordinates.x * last_box_b.coordinates.x, 25272)
}
