use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod knapsack;

struct Example<'a> {
	file_name: &'a str,
	optimal_solution_value: u32,
}

fn main() {
	let examples = [
		Example {file_name: "test_cases/example_1.txt", optimal_solution_value: 8},
		Example {file_name: "test_cases/example_2.txt", optimal_solution_value: 1398904},
		Example {file_name: "test_cases/knapsack1.txt", optimal_solution_value: 0},
		Example {file_name: "test_cases/knapsack_big.txt", optimal_solution_value: 0},
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

	let (knapsack_size, items) = read_problem(&mut reader, &file_name);
	let mut solver = knapsack::Solver::new(items, knapsack_size);
	let computed_optimal_solution = solver.solve();
	verify_example(example, computed_optimal_solution, knapsack_size);
}

fn verify_example(example: &Example, computed_optimal_value: u32, knapsack_size: u32) {
	if example.optimal_solution_value > 0 {
		println!("For knapsack size {} found an optimal solution of {} and expected {}",
				 knapsack_size, computed_optimal_value, example.optimal_solution_value);
		assert_eq!(example.optimal_solution_value, computed_optimal_value);
	}
	else {
		println!("For knapsack size {}, found an optimal solution of {}", knapsack_size, computed_optimal_value);
	}
}

fn read_problem(reader: &mut BufReader<&mut File>, file_name: &std::path::Display) -> (u32, Vec<knapsack::Item>) {
	let (knapsack_size, item_count) = read_problem_size(reader);

	let mut items: Vec<knapsack::Item> = Vec::with_capacity(item_count as usize);

	for line in reader.lines() {
		match line {
			Err(why) => panic!("couldn't read {}: {}", file_name, Error::description(&why)),
			Ok(line_contents) => {
				items.push(read_item_from_line(line_contents.trim().as_ref()));
			}
		}
	}

	println!("In file {}, read a knapsack problem of: {} weight, {} items", file_name, knapsack_size, items.len());
	(knapsack_size, items)
}

fn read_problem_size(reader: &mut BufReader<&mut File>) -> (u32, u32) {
	let mut line = String::new();
	reader.read_line(&mut line).ok().expect("Could not read line");
	let fields = line.trim().split(" ").collect::<Vec<&str>>();

	(
		fields[0].parse::<u32>().unwrap(),
		fields[1].parse::<u32>().unwrap(),
	)
}

fn read_item_from_line(line: &str) -> knapsack::Item {
	let fields = line.split(" ").collect::<Vec<&str>>();
	knapsack::Item::new(
		fields[1].parse::<u32>().unwrap(),
		fields[0].parse::<u32>().unwrap(),
	)
}