use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, Display};
use std::cmp;

use graph::*;

pub fn build_implication_graph_from_file(file_name: &str) -> (usize, Vec<DirectedEdge>, Vec<Node>) {
	let path = Path::new(file_name);
	let display_name = path.display();

	let mut file = match File::open(&path) {
			Err(why) => panic!("couldn't open {}: {}", display_name,
							   Error::description(&why)),
			Ok(file) => file,
		};
	let mut reader = BufReader::new(&mut file);

	read_implication_graph(&mut reader, &display_name)
}

fn read_implication_graph(reader: &mut BufReader<&mut File>, file_name: &Display) -> (usize, Vec<DirectedEdge>, Vec<Node>) {
	let edge_count = read_edge_count(reader);

	let mut implications: Vec<(i32, i32)> = Vec::with_capacity(edge_count);
	let mut max_node = 0;

	for line in reader.lines() {
		match line {
			Err(why) => panic!("couldn't read {}: {}", file_name, Error::description(&why)),
			Ok(line_contents) => {
				let (a,b) = read_implication_from_line(line_contents.trim().as_ref());
				max_node = cmp::max(max_node, cmp::max(a.abs(), b.abs()));
				implications.push((a,b));
			}
		}
	}
	
	let (edges, node_builders) = build_edges_and_node_builders(&implications, max_node as usize);
	let nodes = build_nodes(&edges, &node_builders);

	println!("In file {}, read an implication graph of {} edges and {} nodes", file_name, edges.len(), nodes.len());
	(edge_count, edges, nodes)
}

fn initialize_node_builders(capacity: usize, node_builders: &mut Vec<NodeBuilder>) {
	for i in 0 .. capacity {
		node_builders.push(NodeBuilder::new(i));
	}
}

fn build_edges_and_node_builders(implications: &Vec<(i32,i32)>, max_node: usize) -> (Vec<DirectedEdge>, Vec<NodeBuilder>) {
	let edge_count = implications.len() * 2;

	let mut edges: Vec<DirectedEdge> = Vec::with_capacity(edge_count);
	let mut node_builders: Vec<NodeBuilder> = Vec::with_capacity(max_node * 2);
	initialize_node_builders(max_node * 2, &mut node_builders);

	for &(a, b) in implications.iter() {
		let (first_implication, second_implication) = edges_from_implication(a, b, max_node);
		node_builders[first_implication.a].edge_count += 1;
		node_builders[second_implication.a].edge_count += 1;

		edges.push(first_implication);
		edges.push(second_implication);
	}

	(edges, node_builders)
}

fn build_nodes(edges: &Vec<DirectedEdge>, node_builders: &Vec<NodeBuilder>) -> Vec<Node> {
	let mut nodes: Vec<Node> = node_builders.iter().map(|builder| { Node::new(builder) }).collect();
	for edge in edges.iter() { nodes[edge.a].edges.push(edge.clone()); }

	nodes
}

// See http://en.wikipedia.org/wiki/Implication_graph about constructing edges in an implication graph.
// Input is structured so that a positive index means in the condition that variable is true and a negative
// index means that variable is false
fn edges_from_implication(a: i32, b: i32, max_node: usize) -> (DirectedEdge, DirectedEdge) {
	let first_true_index = (a.abs() - 1) as usize; // Subtract 1 because indices come in as 1-based, and we want to use 0-based indices
	let first_false_index = first_true_index + max_node; // Complement node to a true node is offset by max_node
	let second_true_index = (b.abs() - 1) as usize;
	let second_false_index = second_true_index + max_node; // Complement node to a true node is offset by max_node

	match (a > 0, b > 0) {
		(true, true) => (DirectedEdge { a: first_false_index, b: second_true_index }, DirectedEdge { a: second_false_index, b: first_true_index }),
		(true, false) => (DirectedEdge { a: first_false_index, b: second_false_index }, DirectedEdge { a: second_true_index, b: first_true_index }),
		(false, true) => (DirectedEdge { a: first_true_index, b: second_true_index }, DirectedEdge { a: second_false_index, b: first_false_index }),
		(false, false) => (DirectedEdge { a: first_true_index, b: second_false_index }, DirectedEdge { a: second_true_index, b: first_false_index }),
	}
}

fn read_edge_count(reader: &mut BufReader<&mut File>) -> usize {
	let mut line = String::new();
	reader.read_line(&mut line).ok().expect("Could not read line");
	let fields = line.trim().split(" ").collect::<Vec<&str>>();

	fields[0].parse::<usize>().unwrap()
}

fn read_implication_from_line(line: &str) -> (i32, i32) {
	let fields = line.split(" ").collect::<Vec<&str>>();
	(
		fields[0].parse::<i32>().unwrap(),
		fields[1].parse::<i32>().unwrap()
	)
}


#[test]
fn test_implication_graph_building() {
	let (edge_count, edges, nodes) = build_implication_graph_from_file("test_cases/example_1.txt");
	assert_eq!(11, edge_count);
	assert_eq!(22, edges.len());
	assert_eq!(14, nodes.len());

	assert_eq!(0, nodes[0].edges.len());
	assert_eq!(3, nodes[7].edges.len());
	assert_eq!(2, nodes[3].edges.len());
	assert_eq!(1, nodes[10].edges.len());
}