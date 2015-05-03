#![feature(core)]

mod gosper_subset;
mod graph;
mod salesman_builder;
mod solver;


struct Example<'a> {
	file_name: &'a str,
	route_distance: f32,
}

fn main() {
	let examples = [
		Example {file_name: "test_cases/rectangle.txt", route_distance: 10f32},
		Example {file_name: "test_cases/medium.txt", route_distance: 3.50116f32},
		Example {file_name: "test_cases/tsp.txt", route_distance: std::f32::MAX},
	];

	for example in examples.iter() {
		run_example(example);
	}
}

fn run_example(example: &Example) {
	let (problem_size, initial_edges, salesman_edges) = salesman_builder::build_salesman_from_file(example.file_name);
	let mut solver = solver::Solver::new(problem_size, initial_edges, salesman_edges);
	let found_disatance = solver.solve();

	verify_example(example, found_disatance);
}

fn verify_example(example: &Example, found_distance: f32) {
	if example.route_distance != std::f32::MAX {
		println!("For example {}, expected route of length {}, found route of length {:?}",
				 example.file_name, example.route_distance, found_distance);
		let difference = (example.route_distance - found_distance).abs();
		if (difference > 0.0001f32) {
			assert_eq!(example.route_distance, found_distance);
		}
	}
	else {
		println!("For example {} found path of length {}",
				 example.file_name, found_distance);
	}
}
