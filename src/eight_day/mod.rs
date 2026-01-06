use std::fs::read_to_string;
use std::hash::Hash;
use std::{cmp::Ordering, collections::HashMap};

use itertools::{Itertools, iproduct};

#[cfg(test)]
mod tests;

fn parse_input(input: String) -> Vec<JunctionBox> {
    input
        .lines()
        .enumerate()
        .map(|(index, string_coordinates)| JunctionBox {
            index,
            coordinates: JunctionBoxCoordinates::from_string_tuple(string_coordinates),
        })
        .collect::<Vec<JunctionBox>>()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JunctionBoxCoordinates {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionBoxCoordinates {
    fn distance(&self, other: &Self) -> isize {
        (self.x as isize - other.x as isize).pow(2)
            + (self.y as isize - other.y as isize).pow(2)
            + (self.z as isize - other.z as isize).pow(2)
    }

    fn from_string_tuple(s: &str) -> Self {
        let split_string = s.split(",").into_iter().collect::<Vec<&str>>();
        assert_eq!(split_string.len(), 3);
        JunctionBoxCoordinates {
            x: split_string[0]
                .parse()
                .expect("The x coordinate to be a valid int"),
            y: split_string[1]
                .parse()
                .expect("The y coordinate to be a valid int"),
            z: split_string[2]
                .parse()
                .expect("The z coordinate to be a valid int"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JunctionBox {
    coordinates: JunctionBoxCoordinates,
    index: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Circuit {
    id: usize,
}

struct CircuitMapping {
    junction_box_id_to_circuit: HashMap<usize, Circuit>,
    junction_box_id_to_junction_box: HashMap<usize, JunctionBox>,
}

impl CircuitMapping {
    fn new(junction_boxes: &Vec<JunctionBox>) -> Self {
        Self {
            junction_box_id_to_circuit: HashMap::from_iter(junction_boxes.iter().map(
                |junction_box| {
                    (
                        junction_box.index,
                        Circuit {
                            id: junction_box.index,
                        },
                    )
                },
            )),
            junction_box_id_to_junction_box: HashMap::from_iter(
                junction_boxes
                    .iter()
                    .map(|junction_box| (junction_box.index, junction_box.clone())),
            ),
        }
    }

    fn get_circuit_of_junction_box(&self, junction_box_id: &usize) -> &Circuit {
        self.junction_box_id_to_circuit
            .get(junction_box_id)
            .expect("The mapping to be complete")
    }

    fn get_circuit_boxes(&self, circuit_id: usize) -> Vec<usize> {
        // Get the junction boxes ids of a given circuit
        self.junction_box_id_to_circuit
            .iter()
            .filter(|(_, circuit)| circuit.id == circuit_id)
            .map(|(&junction_box_id, _)| junction_box_id)
            .collect()
    }

    fn get_circuit(&self, circuit_id: usize) -> Option<&Circuit> {
        self.junction_box_id_to_circuit
            .values()
            .filter(|&circuit| circuit.id == circuit_id)
            .nth(0)
    }

    fn merge_circuits(&mut self, circuit_id_a: usize, circuit_id_b: usize) -> () {
        let circuit_a = self
            .get_circuit(circuit_id_a)
            .expect("The circuit to not be merged")
            .clone();
        self.get_circuit_boxes(circuit_id_b)
            .iter()
            .for_each(|&junction_box_id| {
                self.junction_box_id_to_circuit
                    .insert(junction_box_id, circuit_a.to_owned());
            });
    }

    fn get_junction_box(&self, junction_box_id: &usize) -> &JunctionBox {
        self.junction_box_id_to_junction_box
            .get(junction_box_id)
            .expect("The mapping to be complete")
    }

    fn build_connections(
        &mut self,
        distance_mapping: &DistanceMapping,
        n_max_connections: Option<usize>,
        // Build the connections over the circuit mapping based on closest distance.
        // If n_max_connections is Some, stops after n connections and returns None.
        // If n_max_connection is None, break only when there is one single circuit connecting all junction boxes
        // and return the last two junction boxes that were connected.
    ) -> Option<(&JunctionBox, &JunctionBox)> {
        let mut distance_iterator = distance_mapping
            .mapping
            .iter()
            .filter(|((k1, k2), _)| k1 != k2)
            .sorted_by_key(|(_, value)| *value);
        let mut n_connections = 0;
        while match n_max_connections {
            Some(max_connections) => n_connections < max_connections,
            None => true,
        } {
            let ((idx_a, idx_b), _) = distance_iterator.next().expect(
                "The distances to not finish before we have the correct number of connections",
            );
            let circuit_a = self.get_circuit_of_junction_box(idx_a);
            let circuit_b = self.get_circuit_of_junction_box(idx_b);
            n_connections += 1;
            self.merge_circuits(circuit_a.id, circuit_b.id);
            if self.get_circuits_size().len() == 1 {
                return Some((self.get_junction_box(idx_a), self.get_junction_box(idx_b)));
            }
        }
        None
    }

    fn get_circuits_size(&self) -> HashMap<&Circuit, usize> {
        self.junction_box_id_to_circuit.values().counts()
    }
}

struct DistanceMapping {
    // Mapping of the distances between two junction boxes
    mapping: HashMap<(usize, usize), isize>,
}

impl DistanceMapping {
    fn set(&mut self, index_a: usize, index_b: usize, v: isize) -> () {
        // Always insert the distance so the tuple key is ordered
        match index_a
            .partial_cmp(&index_b)
            .expect("indexes to be comparable")
        {
            Ordering::Greater => {
                self.mapping.insert((index_a, index_b), v);
            }
            Ordering::Less => {
                self.mapping.insert((index_b, index_a), v);
            }
            Ordering::Equal => (),
        };
    }
}

pub fn main() {
    let input =
        parse_input(read_to_string("data/day_8.txt").expect("The input file to be readable"));
    let mut circuit_mapping = CircuitMapping::new(&input);
    let mut distances = DistanceMapping {
        mapping: HashMap::new(),
    };
    iproduct!(input.iter(), input.iter()).for_each(|(box_a, box_b)| {
        if box_a.index != box_b.index {
            distances.set(
                box_a.index,
                box_b.index,
                box_a.coordinates.distance(&box_b.coordinates),
            )
        }
    });
    circuit_mapping.build_connections(&distances, Some(1000));
    let result_first_part: usize = circuit_mapping
        .get_circuits_size()
        .values()
        .sorted()
        .rev()
        .take(3)
        .product();
    println!(
        "The result of the first part (product of the size of the 3 largest circuits after 1000 connections is : {}",
        result_first_part
    );
    let result_second_part = circuit_mapping
        .build_connections(&distances, None)
        .and_then(|(box_a, box_b)| Some(box_a.coordinates.x * box_b.coordinates.x))
        .expect("A single circuit to eventually connect all junction boxes");
    println!(
        "The result of the second part (product of the X coordinate of the last two junction boxes to be connected is : {}",
        result_second_part
    )
}
