use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use graph::core::*;
use petgraph::unionfind::UnionFind;

extern crate graph;
extern crate petgraph;

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
		Example {file_name: "test_cases/clustering.txt", cluster_count: -1, distance: -1},
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

	let (node_count, mut edges) = read_graph(&mut reader, &file_name);
	let mut union = UnionFind::<u32>::new(node_count as usize);

	sort_edges_by_weight(&mut edges);
	let distance = run_clustering(node_count, &mut union, &edges, example);
	verify_example(example, distance);
}

fn sort_edges_by_weight(edges: &mut Vec<UndirectedEdge>) {
	edges.sort_by(|a, b| { a.weight.partial_cmp(& b.weight).unwrap() });
//	let weights = edges.iter().map(|edge| { edge.weight }).collect::<Vec<i32>>();
//	println!("{:?}", weights);
}

// Runs kruskal-equivalent collection over the list of edges until the union count equals the
// target in the example
fn run_clustering(node_count: i32, u: &mut UnionFind<u32>, edges: &Vec<UndirectedEdge>, example: &Example) -> i32 {
	let mut union_count = node_count;
	let mut edges_added = 0;
	let target_clusters = if example.cluster_count > 0 { example.cluster_count } else { 4 };
	for edge in edges.iter() {
		if u.union(edge.a, edge.b) {
			union_count -= 1;
//			println!("Added edge weight weight {} to the union. {}->{} clusters remaining.", edge.weight, (union_count + 1), union_count);
		}
		else {

		}
		edges_added += edges_added;
		if union_count <= target_clusters { break; }
	}

//	println!("{:?}", u.clone().into_labeling());

	let mut remaining_distance = 0;
	for i in edges_added..edges.len() {
		let ref edge = edges[i];
		if u.union(edge.a, edge.b) {
			remaining_distance = edge.weight;
			break;
//			println!("Edge with weight {} remainded", edge.weight);
//			println!("{:?}", u.clone().into_labeling());
		}
	}

	remaining_distance
}

fn verify_example(example: &Example, distance: i32) {
	if example.cluster_count > 0 {
		println!("For {} clusters, found a distance of {} and expected {}", example.cluster_count, distance, example.distance);
		assert_eq!(example.distance, distance);
	}
	else {
		println!("For 4 clusters, found a distance of {}", distance);
	}
}

fn read_graph(reader: &mut BufReader<&mut File>, file_name: &std::path::Display) -> (i32, Vec<UndirectedEdge>) {
	let node_count = read_graph_size(reader);

	let mut edges: Vec<UndirectedEdge> = Vec::with_capacity((node_count * 2) as usize);

	for line in reader.lines() {
		match line {
			Err(why) => panic!("couldn't read {}: {}", file_name, Error::description(&why)),
			Ok(line_contents) => {
				edges.push(read_edge_from_line(line_contents.trim().as_ref()));
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
		fields[0].parse::<u32>().unwrap() - 1,
		fields[1].parse::<u32>().unwrap() - 1,
	)
}