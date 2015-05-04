struct Example<'a> {
	file_name: &'a str,
	solveable: Option<bool>,
}

fn main() {
	let examples = [
		Example {file_name: "test_cases/2sat1.txt", solveable: None},
		Example {file_name: "test_cases/2sat2.txt", solveable: None},
		Example {file_name: "test_cases/2sat3.txt", solveable: None},
		Example {file_name: "test_cases/2sat4.txt", solveable: None},
		Example {file_name: "test_cases/2sat5.txt", solveable: None},
		Example {file_name: "test_cases/2sat6.txt", solveable: None},
	];

	for example in examples.iter() {
		run_example(example);
	}
}

fn run_example(example: &Example) {
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
