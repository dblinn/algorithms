mod bits;
mod buckets;

use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use petgraph::unionfind::UnionFind;
use bits::Bits;
use buckets::Buckets;

extern crate petgraph;

struct Example<'a> {
	file_name: &'a str,
	cluster_count: u32,
}

fn main() {
	let examples = [
//		Example {file_name: "test_cases/test_1000.txt", cluster_count: 989},
//		Example {file_name: "test_cases/test_10000.txt", cluster_count: 9116},
		Example {file_name: "test_cases/clustering_big.txt", cluster_count: 0},
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

	let (node_count, mut nodes) = read_graph(&mut reader, &file_name);
	assert_eq!(node_count as usize, nodes.len());
	let mut union = UnionFind::<u32>::new(node_count as usize);

	println!("Building buckets");
	let buckets = build_buckets(&mut nodes, node_count);
	println!("Clustering");
	let cluster_count = bucket_union_nodes(&nodes, &mut union, &buckets);
//	let cluster_count = brute_force_union_nodes(&nodes, &mut union);
	verify_example(example, cluster_count);
	// iterate the list of nodes, for all related nodes identified by the bucket, union them if hamming distance <= 2
	// verify the example
}

fn build_buckets(nodes: &mut Vec<u32>, node_count: u32) -> Buckets {
//	let mut buckets = Buckets::new(nodes, Box::new(|x| { Bits::count(x) }), 0, node_count as usize, 24);
	let mut buckets = Buckets::new(nodes, Box::new(|x| { 12 + Bits::count(&(x & 0xCCCCCC)) - Bits::count(&(x & 0x333333)) }), 0, node_count as usize, 24);
//	buckets.print_contents();

	for ref mut item in buckets.ranges.iter_mut() {
		item.sub_bucket(nodes, Box::new(|x| { 12 + Bits::count(&(x & 0x00000FFF)) - Bits::count(&(x & 0x00FFF000)) }), 24);
		match item.sub_buckets.as_mut() {
			Some(x) => {
				for ref mut sub_item in x.ranges.iter_mut() {
					sub_item.sub_bucket(nodes, Box::new(|x| { 12 + Bits::count(&(x & 0x00555555)) - Bits::count(&(x & 0x00AAAAAA)) }), 24);
					match sub_item.sub_buckets.as_mut() {
						Some(y) => {
							for ref mut sub_sub_item in y.ranges.iter_mut() {
								sub_sub_item.sub_bucket(nodes, Box::new(|x| { 12 + Bits::count(&(x & 0x00F0F0F0)) - Bits::count(&(x & 0x000F0F0F))}), 24);
								match sub_sub_item.sub_buckets.as_mut() {
									Some(z) => {
										for ref mut ultra_item in z.ranges.iter_mut() {
											ultra_item.sub_bucket(nodes, Box::new(|x| { 12 + Bits::count(&(x & 0x003F03F)) - Bits::count(&(x & 0x0FC0FC0))}), 24);
										}
									}
									None => {}
								}
							}
						}
						None => {}
					}
				}
			}
			None => {}
		}
	}

//	let mut v = vec![];
//	buckets.associated_ranges(&mut v, nodes[50000]);
//	let sum = v.iter().fold(0, |total, &(start, end)| { total + end - start });
//	println!("{} {} {:?}", sum, v.len(), v);

	buckets
}

fn bucket_union_nodes(nodes: &Vec<u32>, union: &mut UnionFind<u32>, buckets: &Buckets) -> u32 {
	let node_count = nodes.len();
	let mut cluster_count = nodes.len() as u32;
	let mut ranges : Vec<(usize,usize)> = vec![];

	let mut i: u32 = 0;
	for node in nodes.iter() {
		ranges.clear();
		buckets.associated_ranges(&mut ranges, *node);
		cluster_count -= bucket_union(node, nodes, union, &ranges, i);

		i += 1;
		if node_count > 50000 && (i % 1000 == 0) && i > 0 {
			println!("At iteration {} of {}", i, nodes.len());
		}
	}

	cluster_count
}

// Returns the number of unions performed
fn bucket_union(node: &u32, nodes: &Vec<u32>, union: &mut UnionFind<u32>, ranges: &Vec<(usize,usize)>, node_index: u32) -> u32 {
	if node_index % 1000 == 0 {
		let sum = ranges.iter().fold(0, |total, &(start, end)| { total + end - start });
		println!("Testing against {} of {} values ({} ranges)", sum, nodes.len(), ranges.len());
	}

	let mut cluster_count = 0;
	for range in ranges.iter() {
		let (start,end) = *range;
		for i in start .. end {
			if Bits::hamming_distance(&nodes[i], node) <= 2 && union.union(i as u32, node_index) {
				cluster_count += 1;
			}
		}
	}

	cluster_count
}

//fn brute_force_union_nodes(nodes: &Vec<u32>, union: &mut UnionFind<u32>) -> u32 {
//	let mut cluster_count = nodes.len();
//	for i in 0 .. nodes.len() {
//		for j in (i+1) .. nodes.len() {
//			let ref a = nodes[i];
//			let ref b = nodes[j];
//			if Bits::hamming_distance(a, b) <= 2 {
//				if union.union(i as u32, j as u32) {
//					cluster_count -= 1;
//				}
//			}
//		}
//	}
//
////	let set = (0..nodes.len()).map(|i| union.find(i as u32)).collect::<std::collections::HashSet<_>>();
////	println!("Disjoint parts={:?}", set.len());
//
//	cluster_count as u32
//}

fn verify_example(example: &Example, cluster_count: u32) {
	if example.cluster_count > 0 {
		println!("For example with expected cluster count of {}, found {} clusters", example.cluster_count, cluster_count);
		assert_eq!(example.cluster_count, cluster_count);
	}
	else {
		println!("For main example, found cluster count of {}", cluster_count);
	}
}

fn read_graph(reader: &mut BufReader<&mut File>, file_name: &std::path::Display) -> (u32, Vec<u32>) {
	let (node_count, bits_per_node) = read_graph_size(reader);
	if bits_per_node != 24 { panic!("This solution only works for 24 bit nodes, found {}", bits_per_node); }

	let mut nodes: Vec<u32> = Vec::with_capacity((node_count) as usize);

	for line in reader.lines() {
		match line {
				Err(why) => panic!("couldn't read {}: {}", file_name, Error::description(&why)),
				Ok(line_contents) => {
					nodes.push(read_node_from_line(line_contents.trim().as_ref()));
			}
		}
	}

	println!("In file {}, read a graph size of: {} nodes, {} bits per edge", file_name, node_count, bits_per_node);
	(node_count, nodes)
}

fn read_graph_size(reader: &mut BufReader<&mut File>) -> (u32, u32) {
	let mut line = String::new();
	reader.read_line(&mut line).ok().expect("Could not read line");
	let fields = line.trim().split(" ").collect::<Vec<&str>>();

	(
		fields[0].parse::<u32>().unwrap(),
		fields[1].parse::<u32>().unwrap()
	)
}

fn read_node_from_line(line: &str) -> u32 {
	// read bits using an iterator?
	let mut node = 0u32;
	for c in line.chars() {
		match c {
			'0' => { node <<= 1 },
			'1' => { node |= 1; node <<= 1 },
			_ => {}
		}
	}

//	println!("{} {:b}", line, node);
	node
}