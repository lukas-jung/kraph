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
