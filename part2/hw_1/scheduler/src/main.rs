#![feature(core)]

use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// TODO: Move Job class to separate file, include unit tests in it
// Create Schedule class, moved to separate file, no unit tests
// main.rs creates a schedule and sorts it both ways.

struct Job {
	weight: i32,
	duration: i32,
}

impl Job {
	fn difference(&self) -> i32 {
		self.weight - self.duration
	}

	fn ratio(&self) -> f32 {
		(self.weight as f32) / (self.duration as f32)
	}
}

fn main() {
	// Create a path to the desired file
	let path = Path::new("test_cases/test_1.txt");
	let display = path.display();

	// Open the path in read-only mode, returns `IoResult<File>`
	let mut file = match File::open(&path) {
			// The `desc` field of `IoError` is a string that describes the error
			Err(why) => panic!("couldn't open {}: {}", display,
							   Error::description(&why)),
			Ok(file) => file,
		};

	read_file(&mut file, &display);
	// `file` goes out of scope, and the opened file gets closed
}

fn read_file(file: &mut File, display: &std::path::Display) {
	// Read the file contents into a string, returns `IoResult<String>`
	let mut reader = BufReader::new(file);
	let schedule_size = read_first_line(&mut reader);
	println!("Found a schedule size of: {}", schedule_size);

	for line in reader.lines() {
		match line {
			Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
			Ok(line_contents) => {
				handle_line(line_contents.trim().as_slice());
			}
		}
	}
}

fn read_first_line(reader: &mut BufReader<&mut File>) -> i32 {
	let mut first_line = String::new();
	reader.read_line(&mut first_line);
	handle_first_line(first_line.trim().as_slice())
}

fn handle_first_line(line: &str) -> i32 {
	match line.parse::<i32>() {
		Ok(num) => num,
		Err(why) => {
			println!("Error: {}", Error::description(&why));
			-1
		}
	}
}

fn handle_line(line: &str) {
	println!("{}", line);
//	let fields =  line.split_str(" ").collect::<Vec<&str>>();
}