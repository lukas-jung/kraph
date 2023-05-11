use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::DerefMut,
};

use ordered_float::NotNan;

use crate::{Graph, NodeIx};

pub struct MST<'a, E> {
    start_ix: NodeIx,
    adj_list: Vec<Vec<(NodeIx, &'a E)>>,
    weight: E,
}

pub fn kruskal<N>(graph: &impl Graph<N, NotNan<f64>>) -> NotNan<f64> {
    let mut edges: Vec<_> = graph.get_all_edges().deref_mut().collect();
    edges.sort_by_key(|e| e.2);

    let mut id2set: HashMap<u32, HashSet<NodeIx>> = (0..graph.size().0)
        .map(|i| (i, [NodeIx(i)].into()))
        .collect();
    let mut ix2id: Vec<u32> = (0..graph.size().0).collect();

    fn union(
        target_set_id: u32,
        other_set_id: u32,
        id2set: &mut HashMap<u32, HashSet<NodeIx>>,
        ix2id: &mut Vec<u32>,
    ) {
        for &ix in &id2set[&other_set_id] {
            ix2id[ix] = target_set_id;
        }
        let other_set = id2set.remove(&other_set_id).unwrap();
        id2set.get_mut(&target_set_id).unwrap().extend(other_set);
    }

    let mut result = NotNan::new(0f64).unwrap();
    for edge in edges.iter().copied() {
        let from_id = ix2id[edge.0];
        let to_id = ix2id[edge.1];

        if from_id != to_id {
            let from_set_size = id2set[&from_id].len();
            let to_set_size = id2set[&to_id].len();
            if from_set_size >= to_set_size {
                union(from_id, to_id, &mut id2set, &mut ix2id);
            } else {
                union(to_id, from_id, &mut id2set, &mut ix2id);
            }

            result += edge.2;
        }
    }
    result
}

pub fn prim<N>(graph: &impl Graph<N, NotNan<f64>>) -> NotNan<f64> {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct WeightedEdgeTarget(NotNan<f64>, NodeIx);

    let mut in_mst = vec![false; graph.size().0 as usize];
    let mut fringe = BinaryHeap::new();

    fn add_edge_to_mst<NN>(
        ix: NodeIx,
        graph: &impl Graph<NN, NotNan<f64>>,
        in_mst: &mut Vec<bool>,
        fringe: &mut BinaryHeap<Reverse<WeightedEdgeTarget>>,
    ) {
        in_mst[ix] = true;
        for (weight, other_ix) in graph.get_edges(ix).map(|(o, w)| (*w, o)) {
            if !in_mst[other_ix] {
                fringe.push(Reverse(WeightedEdgeTarget(weight, other_ix)));
            }
        }
    }

    add_edge_to_mst(NodeIx(0), graph, &mut in_mst, &mut fringe);

    let mut result = NotNan::new(0f64).unwrap();
    while let Some(Reverse(WeightedEdgeTarget(weight, other_ix))) = fringe.pop() {
        if !in_mst[other_ix] {
            add_edge_to_mst(other_ix, graph, &mut in_mst, &mut fringe);
            result += weight;
        }
    }

    result
}
