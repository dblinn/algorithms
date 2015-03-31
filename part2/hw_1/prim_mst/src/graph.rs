#[derive(Debug)]
pub struct UndirectedEdge {
	pub weight: i32,
	pub a: i32,
	pub b: i32,
}

#[derive(Debug)]
pub struct NodeNeighbor {
	pub weight: i32,
	pub neighbor: i32,
}

#[derive(Debug)]
pub struct Node {
	pub index: i32,
	pub edges: Box<Vec<NodeNeighbor>>,
}

impl Node {
}

pub struct Graph {
	pub nodes: Box<Vec<Node>>
}

impl Graph {
	pub fn create_nodes(node_count: i32, edges: &Vec<UndirectedEdge>) -> Vec<Node> {
		let mut nodes: Vec<Node> = Vec::with_capacity(node_count as usize);
		for i in 0..node_count {
			nodes.push(Node { index: i, edges: Box::new(Vec::new()) });
		}

		for edge in edges.iter() {
			nodes[edge.a as usize].edges.push(NodeNeighbor { weight: edge.weight, neighbor: edge.b });
			nodes[edge.b as usize].edges.push(NodeNeighbor { weight: edge.weight, neighbor: edge.a });
		}

//		println!("{:?}", nodes);
		nodes
	}
}

//#[test]
//fn test_difference() {
//	assert_eq!((Job {weight: 10, duration: 5}).difference(), 5);
//}
//
//#[test]
//fn test_ratio() {
//	assert_eq!((Job {weight: 10, duration: 5}).ratio(), 2.0 as f32);
//}