pub mod mst;

use ordered_float::NotNan;

use crate::{Graph, NodeIx};

pub fn count_components<N, E>(graph: impl Graph<N, E>) -> u32 {
    let size = graph.size().0;
    let mut visited = vec![false; size as usize];
    let mut count = 0;

    for ix in 0..size {
        if !std::mem::replace(&mut visited[ix as usize], true) {
            let mut stack = vec![NodeIx(ix)];

            while let Some(current) = stack.pop() {
                for (adj_ix, _) in graph.get_edges(current) {
                    if !std::mem::replace(&mut visited[adj_ix], true) {
                        stack.push(adj_ix);
                    }
                }
            }

            count += 1;
        }
    }
    count
}

pub fn nearest_neighbor<N>(graph: &impl Graph<N, NotNan<f64>>, start_ix: NodeIx) -> NotNan<f64> {
    let mut visited = vec![false; graph.size().0 as usize];
    visited[start_ix] = true;

    let mut current_node = start_ix;
    let mut result = NotNan::new(0f64).unwrap();

    while let Some((next_node, weight)) = graph
        .get_edges(current_node)
        .filter(|(node_ix, _)| !visited[*node_ix])
        .min_by_key(|(_, weight)| **weight)
    {
        result += weight;
        visited[next_node] = true;
        current_node = next_node;
    }

    result += graph
        .get_edge_weight(current_node, start_ix)
        .expect("Didn't find an edge back to the start");

    result
}

pub fn double_mst<N>(graph: &impl Graph<N, NotNan<f64>>, start_ix: NodeIx) -> NotNan<f64> {
    let mst = mst::prim(graph, start_ix);

    let mut visited = vec![false; graph.size().0 as usize];

    let mut result = NotNan::default();
    let mut last_node = start_ix;
    for (from_ix, to_ix, weight) in mst.dfs() {
        if !visited[from_ix] {
            visited[from_ix] = true;
            result += weight;
        } else {
            result += graph
                .get_edge_weight(last_node, to_ix)
                .expect("Didn't find shortcut!");
        }
        last_node = to_ix;
    }

    result += graph
        .get_edge_weight(last_node, start_ix)
        .expect("Didn't find an edge back to the start");

    result
}

pub fn brute_force<N>(graph: &impl Graph<N, u64>, start_ix: NodeIx, bnb: bool) -> u64 {
    // let mut ix_set: HashSet<NodeIx> = (0..graph.size().0).map(|i| NodeIx(i)).collect();
    // ix_set.remove(&start_ix);
    struct State {
        visited: Vec<bool>,
        continue_visit: Vec<NodeIx>,
        route: Vec<NodeIx>,
        weight: u64,
    }

    let mut state = State {
        visited: vec![false; graph.size().0 as usize],
        continue_visit: vec![NodeIx(0); graph.size().0 as usize],
        route: vec![start_ix],
        weight: 0,
    };
    state.visited[start_ix] = true;
    state.continue_visit[0] = graph.size();

    impl State {
        fn find_next_step(&self) -> Option<NodeIx> {
            if let Some(&continue_ix) = self.continue_visit.get(self.route.len()) {
                for next_ix in continue_ix.into()..self.visited.len() {
                    if !self.visited[next_ix] {
                        return Some(NodeIx(next_ix as u32));
                    }
                }
            }
            None
        }

        fn take_step<NN>(&mut self, node_ix: NodeIx, graph: &impl Graph<NN, u64>) {
            self.weight += graph
                .get_edge_weight(*self.route.last().unwrap(), node_ix)
                .unwrap();

            self.continue_visit[self.route.len()] = NodeIx(node_ix.0 as u32 + 1);

            self.visited[node_ix] = true;
            self.route.push(node_ix);
        }

        fn step_back<NN>(&mut self, graph: &impl Graph<NN, u64>) {
            if let Some(last_continue) = self.continue_visit.get_mut(self.route.len()) {
                *last_continue = NodeIx(0);
            }

            let back_ix = self.route.pop().expect("Can't step back further!");
            self.visited[back_ix] = false;

            self.weight -= graph
                .get_edge_weight(*self.route.last().unwrap(), back_ix)
                .unwrap();
        }
    }

    let mut min = u64::MAX;
    loop {
        if state.route.len() == usize::from(graph.size()) {
            let full_round = state.weight
                + graph
                    .get_edge_weight(*state.route.last().unwrap(), start_ix)
                    .unwrap();
            if full_round < min {
                min = full_round;
            }
            state.step_back(graph);
        } else if let Some(next_ix) = state.find_next_step() {
            state.take_step(next_ix, graph);
            if bnb && state.weight > min {
                state.step_back(graph);
            }
        } else {
            if state.route.len() < 2 {
                break;
            }
            state.step_back(graph);
        }
    }

    min
}
