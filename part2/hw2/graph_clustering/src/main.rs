#![feature(core)]

use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use graph::core::*;

extern crate graph;

struct Example<'a> {
	file_name: &'a str,
	cluster_count: i32,
	distance: i32,
}

fn main() {
	let examples = [
		Example {file_name: "test_cases/test_1.txt", cluster_count: 2, distance: 6},
		Example {file_name: "test_cases/test_1.txt", cluster_count: 3, distance: 5},
		Example {file_name: "test_cases/test_1.txt", cluster_count: 4, distance: 2},
		Example {file_name: "test_cases/test_2.txt", cluster_count: 2, distance: 4472},
		Example {file_name: "test_cases/test_2.txt", cluster_count: 3, distance: 3606},
		Example {file_name: "test_cases/test_2.txt", cluster_count: 4, distance: 1414},
//		Example {file_name: "test_cases/clustering.txt", cluster_count: -1, distance: -1},
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

	let (node_count, edges) = read_graph(&mut reader, &file_name);
//	let nodes = Graph::create_nodes(node_count, &edges);
//	let mut graph = Graph::new(Box::new(nodes));
//	let mut finder = BruteForceMstGreedyFinder { edges: Box::new(edges) };

//	build_minimum_spanning_tree(&mut graph, &mut finder);
//	examine_graph_correctness(example, &graph);
}

fn read_graph(reader: &mut BufReader<&mut File>, file_name: &std::path::Display) -> (i32, Vec<UndirectedEdge>) {
	let node_count = read_graph_size(reader);

	let mut edges: Vec<UndirectedEdge> = Vec::with_capacity((node_count * 2) as usize);

	for line in reader.lines() {
		match line {
			Err(why) => panic!("couldn't read {}: {}", file_name, Error::description(&why)),
			Ok(line_contents) => {
				edges.push(read_edge_from_line(line_contents.trim().as_slice()));
			}
		}
	}

	println!("In file {}, read a graph size of: {} nodes, {} edges", file_name, node_count, edges.len());
	(node_count, edges)
}

fn read_graph_size(reader: &mut BufReader<&mut File>) -> i32 {
	let mut line = String::new();
	reader.read_line(&mut line).ok().expect("Could not read line");
	let fields = line.trim().split(" ").collect::<Vec<&str>>();

	fields[0].parse::<i32>().unwrap()
}

fn read_edge_from_line(line: &str) -> UndirectedEdge {
	let fields = line.split(" ").collect::<Vec<&str>>();
	UndirectedEdge::new(
		fields[2].parse::<i32>().unwrap(),
		fields[0].parse::<i32>().unwrap() - 1,
		fields[1].parse::<i32>().unwrap() - 1,
	)
}