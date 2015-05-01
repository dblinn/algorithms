mod gosper_subset;
mod graph;

struct Example<'a> {
	file_name: &'a str,
	route_distance: f32,
}

fn main() {
	let examples = [
		Example {file_name: "test_cases/rectangle.txt", route_distance: 10f32},
		Example {file_name: "test_cases/medium.txt", route_distance: 3.50116f32},
//		Example {file_name: "test_cases/tsp.txt", route_distance: std::f32::MAX},
	];

	for example in examples.iter() {
		run_example(example);
	}
}

fn run_example(example: &Example) {
//	let (_, edge_count, nodes) = graph_builder::build_graph_from_file(example.file_name);
}

fn verify_example(example: &Example, found_distance: f32) {
	if example.route_distance != std::f32::MAX {
		println!("For example {}, expected route of length {}, found route of length {:?}",
				 example.file_name, example.route_distance, found_distance);
		assert_eq!(example.route_distance, found_distance);
	}
	else {
		println!("For example {} found path of length {}",
				 example.file_name, found_distance);
	}
}