use graph::*;

mod graph_builder;
mod bellman_ford;
mod dijkstra;
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
	let (node_count, edge_count, nodes) = graph_builder::build_graph_from_file(example.file_name);
}
