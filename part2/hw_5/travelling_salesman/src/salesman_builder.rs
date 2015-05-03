use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, Display};

use graph::*;

pub fn build_salesman_from_file(file_name: &str) -> (usize, Vec<SalesmanEdge>, Vec<Vec<SalesmanEdge>>) {
	let path = Path::new(file_name);
	let display_name = path.display();

	let mut file = match File::open(&path) {
			Err(why) => panic!("couldn't open {}: {}", display_name,
							   Error::description(&why)),
			Ok(file) => file,
		};
	let mut reader = BufReader::new(&mut file);

	read_salesman_problem(&mut reader, &display_name)
}

fn read_salesman_problem(reader: &mut BufReader<&mut File>, file_name: &Display) -> (usize, Vec<SalesmanEdge>, Vec<Vec<SalesmanEdge>>) {
	let point_count = read_problem_size(reader);

	let mut points: Vec<SalesmanPoint> = Vec::with_capacity(point_count);
	let problem_size = point_count - 1;
	let mut start_edges: Vec<SalesmanEdge> = Vec::with_capacity(problem_size);
	let mut graph_edges: Vec<Vec<SalesmanEdge>> = Vec::with_capacity(problem_size);
	for i in 0..problem_size {
		graph_edges.push(Vec::with_capacity(problem_size));
	}

	for line in reader.lines() {
		match line {
			Err(why) => panic!("couldn't read {}: {}", file_name, Error::description(&why)),
			Ok(line_contents) => {
				points.push(read_point_from_line(line_contents.trim().as_ref()));
			}
		}
	}

	build_start_edges(&mut start_edges, &points);
	build_graph_edges(&mut graph_edges, &points);

	println!("In file {}, read a problem of size: {}", file_name, problem_size);
	(problem_size, start_edges, graph_edges)
}

fn build_start_edges(start_edges: &mut Vec<SalesmanEdge>, points: &Vec<SalesmanPoint>) {
	let first_point = points[0];
	for i in 1 .. points.len() {
		let weight = first_point.distance(&points[i]);
		let edge = SalesmanEdge { weight: weight, neighbor: !0 };
		start_edges.push(edge);
	}
}

fn build_graph_edges(graph_edges: &mut Vec<Vec<SalesmanEdge>>, points: &Vec<SalesmanPoint>) {
	for i in 1 .. points.len() {
		let p0 = points[i];

		for j in i .. points.len() {
			let p1 = points[j];
			let weight = p0.distance(&p1);
			graph_edges[i - 1].push(SalesmanEdge { weight: weight, neighbor: j - 1 });
			if (i != j) {
				graph_edges[j - 1].push(SalesmanEdge { weight: weight, neighbor: i - 1});
			}
		}
	}

	// Assert correctness
	for vec in graph_edges { assert_eq!(points.len() - 1, vec.len()); }
}

fn read_problem_size(reader: &mut BufReader<&mut File>) -> usize {
	let mut line = String::new();
	reader.read_line(&mut line).ok().expect("Could not read line");
	let fields = line.trim().split(" ").collect::<Vec<&str>>();

	fields[0].parse::<usize>().unwrap()
}

fn read_point_from_line(line: &str) -> SalesmanPoint {
	let fields = line.split(" ").collect::<Vec<&str>>();
	SalesmanPoint {
		x: fields[0].parse::<f32>().unwrap(),
		y: fields[1].parse::<f32>().unwrap(),
	}
}

#[test]
fn test_build_from_file() {
	let (problem_size, initial_edges, salesman_edges) = build_salesman_from_file("test_cases/rectangle.txt");
	assert_eq!(9, problem_size);
	assert_eq!(1.0f32, initial_edges[0].weight);
	assert_eq!(0f32, salesman_edges[0][0].weight);
	assert_eq!(1.0f32, salesman_edges[0][1].weight);

	assert_eq!(vec![0,1,2,3,4,5,6,7,8], salesman_edges[0].iter().map(|e| {e.neighbor}).collect::<Vec<usize>>());
	assert_eq!(vec![0,1,2,3,4,5,6,7,8], salesman_edges[2].iter().map(|e| {e.neighbor}).collect::<Vec<usize>>());
}