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

pub fn nearest_neighbor<N>(graph: &impl Graph<N, NotNan<f64>>, start_node: NodeIx) -> NotNan<f64> {
    let mut visited = vec![false; graph.size().0 as usize];
    visited[start_node] = true;

    let mut current_node = start_node;
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
        .get_edge_weight(current_node, start_node)
        .expect("Didn't find an edge back to the start");

    result
}
