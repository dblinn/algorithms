use super::graph::Node;
use super::tarjan_scc::Tarjan;

pub struct TwoSat;

impl TwoSat {
	pub fn satisfiable(nodes: &mut Vec<Node>) -> bool {
		// Iterate the implication graph and see if any of the constraints and its complement ended up in the same
		// SCC of the implication graph
		TwoSat::run_tarjan(nodes);

		let (vars, complements) = nodes.split_at(nodes.len() / 2);
		let invalid = vars.iter().zip(complements.iter()).any(|(v, not_v)| {
			v.scc_index() == not_v.scc_index()
		});

		!invalid
	}

	fn run_tarjan(nodes: &mut Vec<Node>) {
		let mut tarjan = Tarjan::new(nodes);
		tarjan.compute_scc();
	}
}