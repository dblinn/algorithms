mod graph;
mod implication_graph_builder;
mod tarjan_scc;
mod two_sat;

struct Example<'a> {
	file_name: &'a str,
	solveable: Option<bool>,
}

fn main() {
	let examples = [
		Example {file_name: "test_cases/example_2.txt", solveable: Some(true)},
		Example {file_name: "test_cases/example_3.txt", solveable: Some(false)},
		Example {file_name: "test_cases/2sat1.txt", solveable: None},
		Example {file_name: "test_cases/2sat2.txt", solveable: None},
		Example {file_name: "test_cases/2sat3.txt", solveable: None},
		Example {file_name: "test_cases/2sat4.txt", solveable: None},
		Example {file_name: "test_cases/2sat5.txt", solveable: None},
		Example {file_name: "test_cases/2sat6.txt", solveable: None},
	];

	let mut results = String::new();
	for example in examples.iter() {
		let satisfiable = run_example(example);
		if example.solveable == None {
			results.push_str(if satisfiable { "1"} else { "0"});
		}
	}

	println!("{}", results);
}

fn run_example(example: &Example) -> bool {
	let (_, _, mut nodes) = implication_graph_builder::build_implication_graph_from_file(example.file_name);

	let satisfiable = two_sat::TwoSat::satisfiable(&mut nodes);
	verify_example(example, satisfiable);
	satisfiable
}

fn verify_example(example: &Example, solveable: bool) {
	if example.solveable != None {
		println!("For example {}, expected solveability of {}, found solveability of {}",
				 example.file_name, example.solveable.unwrap(), solveable);
		assert_eq!(example.solveable.unwrap(), solveable);
	}
	else {
		println!("For example {} found solveability of {}",
				 example.file_name, solveable);
	}
}
