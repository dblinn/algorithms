mod graph;

use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use graph::{UndirectedEdge, Graph, MstGreedyFinder, BruteForceMstGreedyFinder};

struct Example<'a> {
	file_name: &'a str,
	mst_cost: i64,
	mst_ordered_weights: &'a [i32],
}

fn main() {
	let examples = [
		Example {file_name: "test_cases/test_1.txt", mst_cost: 6, mst_ordered_weights: &[1, 2, 3]},
		Example {file_name: "test_cases/test_2.txt", mst_cost: 4, mst_ordered_weights: &[1, 1, 2]},
		Example {file_name: "test_cases/test_3.txt", mst_cost: -16, mst_ordered_weights: &[-10, -1, -8, -3, 6]},
		Example {file_name: "test_cases/edges.txt", mst_cost: -1, mst_ordered_weights: &[]},
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

	let (node_count, edges) = read_edges(&mut reader, &file_name);
	let nodes = Graph::create_nodes(node_count, &edges);
	let mut graph = Graph::new(Box::new(nodes));
	let mut finder = BruteForceMstGreedyFinder { edges: Box::new(edges) };

	build_minimum_spanning_tree(&mut graph, &mut finder);
	examine_graph_correctness(example, &graph);
}

fn read_edges(reader: &mut BufReader<&mut File>, file_name: &std::path::Display) -> (i32, Vec<UndirectedEdge>) {
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

	println!("Read {} edges", edges.len());
	(node_count, edges)
}

fn build_minimum_spanning_tree(graph: &mut Graph, finder: &mut MstGreedyFinder) {
	let initial_edge = finder.minimum_edge();
	graph.nodes[initial_edge.a as usize].in_tree = true;
	finder.remove_related_edges(graph.node(initial_edge.a));

	loop {
		if finder.done() { break; }

		let (node_index, edge_weight) = finder.greedy_node_index(graph);
		graph.mark_in_tree(node_index, edge_weight);
		finder.remove_related_edges(graph.node(node_index))
	}

	assert!(graph.nodes.iter().all(|node| node.in_tree))
}

fn examine_graph_correctness(example: &Example, graph: &Graph) {
	if example.mst_ordered_weights.is_empty() {
		println!("Found cost {}", graph.mst_cost);
	}
	else {
		println!("Expected mst weight {}, found {}", example.mst_cost, graph.mst_cost);
		println!("Expected mst edges {:?}, found {:?}", example.mst_ordered_weights, graph.tree_weights);
	}
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
	UndirectedEdge::new(
		fields[2].parse::<i32>().unwrap(),
		fields[0].parse::<i32>().unwrap() - 1,
		fields[1].parse::<i32>().unwrap() - 1,
	)
}