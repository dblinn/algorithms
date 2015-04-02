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

// Is all this boilerplate really necessary?
impl PartialEq<Node> for Node {
	fn eq(&self, other: &Node) -> bool {
		if self.index != other.index { return false; }
		if self.edges.len() != other.edges.len() { return false; }
		for i in 0..self.edges.len() {
			if (self.edges[i].weight != other.edges[i].weight) { return false; }
			if (self.edges[i].neighbor != other.edges[i].neighbor) { return false; }
		}
		true
	}

	fn ne(&self, other: &Node) -> bool {
		!self.eq(other)
	}
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

#[test]
fn test_create_nodes() {
	let edges = vec![
		UndirectedEdge {weight: 1, a: 0, b: 1},
		UndirectedEdge {weight: -1, a: 0, b: 2},
		UndirectedEdge {weight: 2, a: 2, b: 1},
	];
	let expected_nodes = vec![
		Node {index: 0, edges: Box::new(vec![NodeNeighbor {weight: 1, neighbor: 1}, NodeNeighbor {weight: -1, neighbor: 2}])},
		Node {index: 1, edges: Box::new(vec![NodeNeighbor {weight: 1, neighbor: 0}, NodeNeighbor {weight: 2, neighbor: 2}])},
		Node {index: 2, edges: Box::new(vec![NodeNeighbor {weight: -1, neighbor: 0}, NodeNeighbor {weight: 2, neighbor: 1}])},
		Node {index: 3, edges: Box::new(vec![])},
	];
	let nodes = Graph::create_nodes(4, &edges);
	for i in 0..nodes.len() {
		assert_eq!(nodes[i], expected_nodes[i]);
	}
}
//
//#[test]
//fn test_ratio() {
//	assert_eq!((Job {weight: 10, duration: 5}).ratio(), 2.0 as f32);
//}