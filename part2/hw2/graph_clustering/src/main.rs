#![feature(core)]

use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use graph::graph::{Graph};

extern crate graph;

fn main() {
    println!("Hello, world! {:?}", Graph {n: 32});
}
