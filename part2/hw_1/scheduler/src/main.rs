#![feature(core)]

use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
mod job;

use job::Job;

// Create Schedule class, moved to separate file, no unit tests
// main.rs creates a schedule and sorts it both ways.


fn main() {
	let path = Path::new("test_cases/test_1.txt");
	let display = path.display();

	let mut file = match File::open(&path) {
			// The `desc` field of `IoError` is a string that describes the error
			Err(why) => panic!("couldn't open {}: {}", display,
							   Error::description(&why)),
			Ok(file) => file,
		};

	read_file(&mut file, &display);
}

fn read_file(file: &mut File, display: &std::path::Display) {
	let mut reader = BufReader::new(file);
	let schedule_size = read_schedule_size(&mut reader);
	println!("Found a schedule size of: {}", schedule_size);

	let mut jobs: Vec<Job> = Vec::with_capacity(schedule_size as usize);
	println!("Size of jobs: {}", jobs.len());

	for line in reader.lines() {
		match line {
			Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
			Ok(line_contents) => {
				jobs.push(create_job_from_line(line_contents.trim().as_slice()));
			}
		}
	}
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