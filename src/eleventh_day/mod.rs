use anyhow::Result;
use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    hash::Hash,
};
use thiserror::Error;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Device {
    name: String,
    outputs: HashSet<String>,
}

impl Hash for Device {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Device {
    fn new(input: &str) -> Self {
        let mut split_input = input.split(": ");
        let name = split_input
            .next()
            .expect("The input to contain a name")
            .to_string();
        let outputs = split_input
            .next()
            .expect("The input to contain output devices")
            .split(" ")
            .map(String::from)
            .collect::<HashSet<String>>();
        Device { name, outputs }
    }
}

fn parse_input(input: String) -> HashMap<String, Device> {
    let mut devices = input
        .lines()
        .map(Device::new)
        .map(|device| (device.name.clone(), device))
        .collect::<HashMap<String, Device>>();
    // Append the devices that only have output
    let output_devices = devices
        .values()
        .map(|device| {
            device
                .outputs
                .iter()
                .filter_map(|device_name| {
                    if !devices.contains_key(device_name) {
                        Some(Device {
                            name: device_name.clone(),
                            outputs: HashSet::new(),
                        })
                    } else {
                        None
                    }
                })
                .collect::<HashSet<Device>>()
        })
        .flatten()
        .collect::<HashSet<Device>>();
    output_devices.iter().for_each(|device| {
        devices.insert(device.name.clone(), device.clone());
    });
    devices
}

#[derive(Debug, Error)]
enum PathFindingError {
    #[error("No device named {0} found")]
    MissingStartingDevice(String),
    #[error("graph is not a DAG")]
    GraphIsnotADag,
}

#[derive(Debug)]
enum PathFindingStatus {
    InvalidPathFound,
    ValidPathFound(Vec<HashSet<String>>),
    ClosedLoop,
}

fn find_paths(
    devices: &HashMap<String, Device>,
    from: &str,
    to: &str,
    existing_path: HashSet<&str>,
    required_nodes: &HashSet<&str>,
    // deadends: &mut HashSet<String>,
) -> Result<PathFindingStatus> {
    if existing_path.contains(from) {
        // Then the path has already been explored
        println!("Found closed loop");
        Ok(PathFindingStatus::ClosedLoop)
    // } else if deadends.contains(from) {
    //     Ok(PathFindingStatus::DeadendFound(from.to_string()))
    } else if from == to {
        if required_nodes.is_subset(&existing_path) {
            Ok(PathFindingStatus::ValidPathFound(vec![
                existing_path.into_iter().map(String::from).collect(),
            ]))
        } else {
            Ok(PathFindingStatus::InvalidPathFound)
        }
    } else {
        let device = devices
            .get(from)
            .ok_or(PathFindingError::MissingStartingDevice(from.to_string()))?;
        let mut existing_path = existing_path.clone();
        existing_path.insert(from);

        let mut result = Vec::new();
        for output in device.outputs.iter() {
            if let Ok(path_status) = find_paths(
                &devices,
                output,
                to,
                existing_path.clone(),
                required_nodes,
                // deadends,
            ) {
                match path_status {
                    PathFindingStatus::ValidPathFound(paths) => {
                        for path in paths {
                            result.push(path)
                        }
                    }
                    _ => {}
                }
            };
        }
        // if result.is_empty() {
        //     deadends.insert(from.to_string());
        //     Ok(PathFindingStatus::DeadendFound(from.to_string()))
        // } else {
        Ok(PathFindingStatus::ValidPathFound(result))
        // }
    }
}

struct Graph {
    edges: HashMap<String, Vec<String>>,
    indegrees: HashMap<String, usize>,
}

impl Graph {
    fn new(devices: &HashMap<String, Device>) -> Self {
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        let mut indegrees: HashMap<String, usize> = HashMap::new();
        for device in devices.values() {
            device.outputs.iter().for_each(|output| {
                edges
                    .entry(device.name.to_string())
                    .and_modify(|entry| entry.push(output.to_string()))
                    .or_insert(vec![output.to_string()]);

                indegrees
                    .entry(output.to_string())
                    .and_modify(|entry| *entry += 1)
                    .or_insert(1);
            });
        }
        Graph { edges, indegrees }
    }
}

// Seems like it not acyclic.
fn topological_sort(devices: &HashMap<String, Device>) -> Result<Vec<Device>, PathFindingError> {
    let mut graph = Graph::new(devices);
    let mut sorted_devices = Vec::new();
    let mut start_nodes = devices
        .values()
        .filter(|&device| !graph.indegrees.contains_key(&device.name))
        .cloned()
        .collect::<Vec<Device>>();
    while !start_nodes.is_empty() {
        let device = start_nodes.pop().expect("The start nodes not to be empty");
        sorted_devices.push(device.clone());

        if let Some(edges) = graph.edges.get_mut(&device.name) {
            while !edges.is_empty() {
                let next_node_name = edges.pop().expect("The edges not to be empty");
                let indegree = graph
                    .indegrees
                    .get_mut(&next_node_name)
                    .expect("The indegree to exist");
                *indegree -= 1;
                if *indegree == 0 {
                    start_nodes.push(
                        devices
                            .get(&next_node_name)
                            .expect("The next node to be present in the devices")
                            .clone(),
                    );
                }
            }
        }
    }
    if graph.edges.values().any(|edge| !edge.is_empty()) {
        Err(PathFindingError::GraphIsnotADag)
    } else {
        Ok(sorted_devices)
    }
}

fn find_n_paths(topologically_sorted_nodes: &Vec<Device>, start: &str, out: &str) -> usize {
    let mut ways: HashMap<String, usize> = HashMap::new();
    ways.insert(start.to_string(), 1);
    for node in topologically_sorted_nodes.iter() {
        let node_ways = *ways.entry(node.name.clone()).or_insert(0);
        for neighbor in node.outputs.iter() {
            let n_ways = ways.entry(neighbor.clone()).or_insert(0);
            *n_ways += node_ways
        }
    }
    if let Some(result) = ways.get(out) {
        *result
    } else {
        0
    }
}

fn find_n_paths_with_2_intermediate_steps(
    topologically_sorted_nodes: &Vec<Device>,
    start: &str,
    end: &str,
    step_1: &str,
    step_2: &str,
) -> usize {
    // From a topologically sorted graph, find the number of paths that go from one point to another
    // Returns the number of paths
    // Step 1 and 2 can be reached in any order
    let count_paths = |start, end| find_n_paths(topologically_sorted_nodes, start, end);
    let from_step_1_to_step_2 = count_paths(step_1, step_2);
    if from_step_1_to_step_2 > 0 {
        count_paths(start, step_1) * from_step_1_to_step_2 * count_paths(step_2, end)
    } else {
        // try to reach step 2 first then step 1
        count_paths(start, step_2) * count_paths(step_2, step_1) * count_paths(step_1, end)
    }
}

pub fn main() {
    let devices =
        parse_input(read_to_string("data/day_11.txt").expect("The data to be correctly read"));
    let Ok(PathFindingStatus::ValidPathFound(first_problem_paths)) = find_paths(
        &devices,
        "you",
        "out",
        HashSet::new(),
        &HashSet::new(),
        // &mut HashSet::new(),
    ) else {
        panic!("Expected a valid search result")
    };
    println!(
        "The solution to the first problem (number of path between you and out): {}",
        first_problem_paths.iter().count()
    );

    let topologically_sorted_graph =
        topological_sort(&devices).expect("The nodes to be topologically sorted");
    let n_total_paths = find_n_paths(&topologically_sorted_graph, "svr", "out");
    println!("Number of total paths from svr to out {}", n_total_paths);
    let second_problem_result = find_n_paths_with_2_intermediate_steps(
        &topologically_sorted_graph,
        "svr",
        "out",
        "fft",
        "dac",
    );
    println!(
        "The result to the second problem (number of points from svr to out passing by fft and dac) is {}",
        second_problem_result
    )
}
