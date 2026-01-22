use std::{collections::{BTreeSet, HashSet}, hash::Hash, str::Lines};
use num::{integer::sqrt};

const FILE_CONTENT: &str = include_str!("./input.txt");

fn get_file_lines<'a>() -> Lines<'a> {
    FILE_CONTENT.lines()
}


#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Connection {
    point: Point,
    another_point: Point,
    distance: i64,
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance
            .cmp(&other.distance)
            .then(self.point.cmp(&other.point))
            .then(self.another_point.cmp(&other.another_point))
    }
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance.cmp(&other.distance))
    }
}

pub fn part1() {
    let lines = get_file_lines();
    let points = parse_lines_to_points(lines);
    let circuits = connect_circuits(points);
    let first_3_elements_lenghts_multiplied = multiply_top_3_lenghts(circuits);
    println!("First 3 lengths multiplied: {}", first_3_elements_lenghts_multiplied);
}

pub fn part2() {
    let lines = get_file_lines();
    let points = parse_lines_to_points(lines);
    let last_2_points = find_last_two_points_to_connect(points);
    let last_2_x_coordinates_multiplied = multiply_last_2_x_coordinates(last_2_points);
    println!("Last 2 x coordinates multiplied: {}", last_2_x_coordinates_multiplied);
}

fn parse_lines_to_points(lines: Lines) -> Vec<Point> {
    lines.map(|line| {
        let line_split: Vec<&str> = line.split(",").collect(); 
        let x_cord: i64 = line_split[0].parse().expect("Error parsing input for x");
        let y_cord: i64 = line_split[1].parse().expect("Error parsing input for y");
        let z_cord: i64 = line_split[2].parse().expect("Error parsing input for z");
        Point{
            x: x_cord,
            y: y_cord,
            z: z_cord
        }
    }).collect()
}

fn connect_circuits(points: Vec<Point>) -> Vec<HashSet<Point>> {
    let mut circuits: Vec<HashSet<Point>> = points.iter().map(|&point| {
        let mut set = HashSet::new();
        set.insert(point);
        return set;
    }).collect();

    let mut connections = Vec::new();
    for (index, &point) in points.iter().enumerate() {
        for &another_point in points.iter().take(index + 1) {
            let distance = calculate_point_distance(point, another_point);
            if distance == 0 {
                continue;
            }
            let connection = Connection{point: point, another_point, distance: distance};
            connections.push(connection);
            // println!("{:?} and {:?} are ({}) away", point, another_point, distance);
        }
    }

    connections.sort();

    println!("\n");
    for connection in connections.iter() {
        // println!("The closest point to {:?} is {:?} ({})", connection.point, connection.another_point, connection.distance);
        // println!("{:?} and {:?} are ({}) away", connection.point, connection.another_point, connection.distance);
    }

    display_current_circuits(&circuits);

    let mut remaining_connections = 1000;
    let mut iterator = connections.iter();
    while remaining_connections > 0 {
        let connection = iterator.next().unwrap();
        // println!("\n\nConnection {:?}", connection);

        let first_point_circuit = circuits.iter().find(|circuit| circuit.contains(&connection.point)).unwrap();
        let second_point_circuit = circuits.iter().find(|circuit| circuit.contains(&connection.another_point)).unwrap();
        if first_point_circuit == second_point_circuit {
            // continue;
        }

        let mut merged_circuit = first_point_circuit.clone();
        merged_circuit.extend(second_point_circuit);

        let are_points_inside = |circuit: &HashSet<Point>| circuit.contains(&connection.point) || circuit.contains(&connection.another_point);
        let remaining_circuits: Vec<HashSet<Point>>  = circuits.iter().filter(|&circuit| !are_points_inside (circuit)).cloned().collect();


        circuits = remaining_circuits;
        circuits.push(merged_circuit);
        remaining_connections = remaining_connections - 1;

        display_current_circuits(&circuits);
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    return circuits;
}

fn find_last_two_points_to_connect(points: Vec<Point>) -> Vec<Point> {
    let mut circuits: Vec<HashSet<Point>> = points.iter().map(|&point| {
        let mut set = HashSet::new();
        set.insert(point);
        return set;
    }).collect();

    let mut connections = Vec::new();
    for (index, &point) in points.iter().enumerate() {
        for &another_point in points.iter().take(index + 1) {
            let distance = calculate_point_distance(point, another_point);
            if distance == 0 {
                continue;
            }
            let connection = Connection{point: point, another_point, distance: distance};
            connections.push(connection);
            // println!("{:?} and {:?} are ({}) away", point, another_point, distance);
        }
    }

    connections.sort();

    println!("\n");
    for connection in connections.iter() {
        // println!("The closest point to {:?} is {:?} ({})", connection.point, connection.another_point, connection.distance);
        // println!("{:?} and {:?} are ({}) away", connection.point, connection.another_point, connection.distance);
    }

    display_current_circuits(&circuits);

    let mut last_points_to_connect = Vec::new();
    let mut iterator = connections.iter();
    loop {
        let connection = iterator.next().unwrap();
        println!("\n\nConnection {:?}", connection);

        let first_point_circuit = circuits.iter().find(|circuit| circuit.contains(&connection.point)).unwrap();
        let second_point_circuit = circuits.iter().find(|circuit| circuit.contains(&connection.another_point)).unwrap();
        if first_point_circuit == second_point_circuit {
            // continue;
        }

        let mut merged_circuit = first_point_circuit.clone();
        merged_circuit.extend(second_point_circuit);

        let are_points_inside = |circuit: &HashSet<Point>| circuit.contains(&connection.point) || circuit.contains(&connection.another_point);
        let remaining_circuits: Vec<HashSet<Point>>  = circuits.iter().filter(|&circuit| !are_points_inside (circuit)).cloned().collect();


        circuits = remaining_circuits;
        circuits.push(merged_circuit);

        display_current_circuits(&circuits);

        if circuits.len() == 1 {
            last_points_to_connect.push(connection.point);
            last_points_to_connect.push(connection.another_point);
            break;
        }
    }

    return last_points_to_connect;
}

fn calculate_point_distance(point_a: Point, point_b: Point) -> i64 {
    let x_diff = point_b.x - point_a.x;
    let y_diff = point_b.y - point_a.y;
    let z_diff = point_b.z - point_a.z;

    let distance_squared = x_diff*x_diff + y_diff*y_diff + z_diff*z_diff;
    return distance_squared;
    // let distance = sqrt(distance_squared);
    // return distance;
}

fn display_current_circuits(circuits: &Vec<HashSet<Point>>) {
    // println!("\n");
    for circuit in circuits {
        // println!("Circuit: {:?}", circuit);
    }
}

fn multiply_top_3_lenghts(circuits: Vec<HashSet<Point>>) -> usize {
    let mut circuits_sorted = circuits.clone();
    circuits_sorted.sort_by(|first, second| second.len().cmp(&first.len()));
    display_current_circuits(&circuits_sorted);

    let circuit_lenghts: Vec<usize> = circuits_sorted.iter().map(|circuit| circuit.len()).collect();
    println!("Circuit lens: {:?}", circuit_lenghts);

    let mut buffer = 1;
    let mut iterator = circuits_sorted.iter();
    for i in 1..=3 {
        let circuit = iterator.next().unwrap();
        println!("Circuit {} len: {}", i, circuit.len());
        buffer *= circuit.len();
    }
    return buffer
}

fn multiply_last_2_x_coordinates(points: Vec<Point>) -> i64 {
    return points[0].x * points[1].x;
}
