use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use graph::*;

mod bellman_ford;
mod djikstra;
mod johnson;
mod graph;

struct Example<'a> {
	file_name: &'a str,
	shortest_shortest_path: i32,
}

fn main() {
	let examples = [
		Example {file_name: "test_cases/example_1.txt", shortest_shortest_path: -10003},
		Example {file_name: "test_cases/example_2.txt", shortest_shortest_path: -6},
//		Example {file_name: "test_cases/g1.txt", shortest_shortest_path: i32::max_value()},
//		Example {file_name: "test_cases/g2.txt", shortest_shortest_path: i32::max_value()},
//		Example {file_name: "test_cases/g3.txt", shortest_shortest_path: i32::max_value()},
//		Example {file_name: "test_cases/large.txt", shortest_shortest_path: i32::max_value()},
	];

	for example in examples.iter() {
		run_example(example);
	}
}

fn run_example(example: &Example) {
	let path = Path::new(example.file_name);
	let file_name = path.display();

	let mut file = match File::open(&path) {
			Err(why) => panic!("couldn't open {}: {}", file_name,
							   Error::description(&why)),
			Ok(file) => file,
		};
	let mut reader = BufReader::new(&mut file);

	let (node_count, edge_count, nodes) = read_graph(&mut reader, &file_name);
}

fn read_graph(reader: &mut BufReader<&mut File>, file_name: &std::path::Display) -> (usize, usize, Vec<Node>) {
	let (node_count, edge_count) = read_problem_size(reader);

	let mut builders: Vec<NodeBuilder> = Vec::with_capacity(node_count);
	let mut edges: Vec<DirectedEdge> = Vec::with_capacity(edge_count);
	for i in 0..node_count {
		builders.push(NodeBuilder::new(i));
	}

	for line in reader.lines() {
		match line {
			Err(why) => panic!("couldn't read {}: {}", file_name, Error::description(&why)),
			Ok(line_contents) => {
				let edge = read_edge_from_line(line_contents.trim().as_ref());
				builders[edge.a].outgoing_edge_count += 1;
				builders[edge.b].incoming_edge_count += 1;
				edges.push(edge);
			}
		}
	}

	let mut nodes: Vec<Node> = builders.into_iter().map(|builder| { builder.to_node() }).collect();
	for edge in edges.iter() {
		nodes[edge.a].out_edges.push(*edge);
		nodes[edge.b].in_edges.push(*edge);
	}

	println!("In file {}, read a graph of size: {} nodes and {} edges", file_name, nodes.len(), edge_count);
	(node_count, edge_count, nodes)
}

fn read_problem_size(reader: &mut BufReader<&mut File>) -> (usize, usize) {
	let mut line = String::new();
	reader.read_line(&mut line).ok().expect("Could not read line");
	let fields = line.trim().split(" ").collect::<Vec<&str>>();

	(
		fields[0].parse::<usize>().unwrap(),
		fields[1].parse::<usize>().unwrap(),
	)
}

fn read_edge_from_line(line: &str) -> DirectedEdge {
	let fields = line.split(" ").collect::<Vec<&str>>();
	DirectedEdge::new(
		fields[0].parse::<usize>().unwrap() - 1,
		fields[1].parse::<usize>().unwrap() - 1,
		fields[2].parse::<i32>().unwrap(),
	)
}