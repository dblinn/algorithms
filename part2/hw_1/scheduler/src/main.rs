#![feature(core)]

use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
mod job;
mod schedule;

use job::Job;
use schedule::Schedule;

struct Example<'a> {
	file_name: &'a str,
	expected_ratio: i64,
	expected_difference: i64,
}

fn main() {
	let examples = [
		Example {file_name: "test_cases/test_1.txt", expected_ratio: 31814, expected_difference: 31814},
		Example {file_name: "test_cases/test_2.txt", expected_ratio: 60213, expected_difference: 61545},
		Example {file_name: "test_cases/test_3.txt", expected_ratio: 674634, expected_difference: 688647},
		Example {file_name: "test_cases/jobs.txt", expected_ratio: -1, expected_difference: -1},
	];

	for example in examples.iter() {
		run_example(example);
	}
}

fn run_example(example: &Example) {
	let path = Path::new(example.file_name);
	let display = path.display();

	let mut file = match File::open(&path) {
		Err(why) => panic!("couldn't open {}: {}", display,
						   Error::description(&why)),
		Ok(file) => file,
	};

	let mut schedule = Box::new(read_file(&mut file, &display));
	examine_schedule_correctness(example, &mut *schedule);
}

fn examine_schedule_correctness(example: &Example, schedule: &mut Schedule) {
	if example.expected_difference >= 0 {
		assert_schedule_correctness(example, schedule);
	}
	else {
		print_schedule_values(schedule);
	}
}

fn print_schedule_values(schedule: &mut Schedule) {
	schedule.sort_by_difference();
	println!("Found difference of {}", schedule.total_weighted_completion_time());

	schedule.sort_by_ratio();
	println!("Found ratio of {}", schedule.total_weighted_completion_time());
	println!("");
}

fn assert_schedule_correctness(example: &Example, schedule: &mut Schedule) {
	schedule.sort_by_difference();
	println!("Expected difference of {}, Found {}", example.expected_difference, schedule.total_weighted_completion_time());
	if schedule.total_weighted_completion_time() != example.expected_difference { panic!("Difference failed") }

	schedule.sort_by_ratio();
	println!("Expected ratio of {}, Found {}", example.expected_ratio, schedule.total_weighted_completion_time());
	if schedule.total_weighted_completion_time() != example.expected_ratio { panic!("Ratio failed") }
	println!("");
}

fn read_file(file: &mut File, display: &std::path::Display) -> Schedule {
	let mut reader = BufReader::new(file);
	let schedule_size = read_schedule_size(&mut reader);
	println!("In file {}, read a schedule size of: {}", display, schedule_size);

	let mut jobs: Vec<Job> = Vec::with_capacity(schedule_size as usize);

	for line in reader.lines() {
		match line {
			Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
			Ok(line_contents) => {
				jobs.push(create_job_from_line(line_contents.trim().as_slice()));
			}
		}
	}

	Schedule { jobs: Box::new(jobs) }
}

fn read_schedule_size(reader: &mut BufReader<&mut File>) -> i32 {
	let mut first_line = String::new();
	reader.read_line(&mut first_line).ok().expect("Could not read line");
	first_line.trim().parse::<i32>().unwrap()
}

fn create_job_from_line(line: &str) -> Job {
	let fields = line.split(" ").collect::<Vec<&str>>();
	Job {
		weight: fields[0].parse::<i32>().unwrap(),
		duration: fields[1].parse::<i32>().unwrap()
	}
}