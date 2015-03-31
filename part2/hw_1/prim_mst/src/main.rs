#![feature(core)]

mod graph;

use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use graph::{UndirectedEdge, Node, Graph};

struct Example<'a> {
	file_name: &'a str,
	mst_cost: i64,
	mst_ordered_weights: &'a [i32],
}

fn main() {
	let examples = [
		Example {file_name: "test_cases/test_1.txt", mst_cost: 31814, mst_ordered_weights: &[1, 2, 3]},
		Example {file_name: "test_cases/test_2.txt", mst_cost: 60213, mst_ordered_weights: &[1, 1, 2]},
		Example {file_name: "test_cases/test_3.txt", mst_cost: 674634, mst_ordered_weights: &[-10, -1, -8, -3, 6]},
//		Example {file_name: "test_cases/edges.txt", mst_cost: -1, mst_ordered_weights: &[]},
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

	build_graph(&mut reader, &file_name);
//	let mut schedule = Box::new(build_graph(&mut reader, &display));
//	examine_schedule_correctness(example, &mut *schedule);
}

fn build_graph(reader: &mut BufReader<&mut File>, file_name: &std::path::Display) {
	let (node_count, edge_count) = read_graph_size(reader);
	println!("In file {}, read a graph size of: {} nodes, {} edges", file_name, node_count, edge_count);

	let mut edges: Vec<UndirectedEdge> = Vec::with_capacity(edge_count as usize);

	for line in reader.lines() {
		match line {
			Err(why) => panic!("couldn't read {}: {}", file_name, Error::description(&why)),
			Ok(line_contents) => {
				edges.push(read_edge_from_line(line_contents.trim().as_slice()));
			}
		}
	}

	let mut nodes = Graph::create_nodes(node_count, &edges);
//	nodes = Graph::create_nodes(node_count, edges);

	println!("Read {} edges", edges.len());
//	Schedule { edges: Box::new(edges) }
}

fn read_graph_size(reader: &mut BufReader<&mut File>) -> (i32, i32) {
	let mut line = String::new();
	reader.read_line(&mut line).ok().expect("Could not read line");
	let fields = line.trim().split(" ").collect::<Vec<&str>>();

	(
		fields[0].parse::<i32>().unwrap(),
	 	fields[1].parse::<i32>().unwrap()
	)
}

fn read_edge_from_line(line: &str) -> UndirectedEdge {
	let fields = line.split(" ").collect::<Vec<&str>>();
	UndirectedEdge {
		weight: fields[2].parse::<i32>().unwrap(),
		a: fields[0].parse::<i32>().unwrap() - 1,
		b: fields[1].parse::<i32>().unwrap() - 1,
	}
}