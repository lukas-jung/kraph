use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::DerefMut,
};

use ordered_float::NotNan;

use crate::{Graph, NodeIx};

pub struct Mst<E> {
    start_ix: NodeIx,
    adj_list: Vec<Vec<(NodeIx, E)>>,
    weight: E,
}

pub struct MstDfsIter<'a, E> {
    mst: &'a Mst<E>,
    stack: Vec<(NodeIx, NodeIx, E)>,
}

impl<E> Iterator for MstDfsIter<'_, E>
where
    E: Copy,
{
    type Item = (NodeIx, NodeIx, E);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((prev_ix, current_ix, weight)) = self.stack.pop() {
            self.stack.extend(
                self.mst.adj_list[current_ix]
                    .iter()
                    .map(|(to_ix, weight)| (current_ix, *to_ix, *weight)),
            );
            Some((prev_ix, current_ix, weight))
        } else {
            None
        }
    }
}

impl<E> Mst<E>
where
    E: Copy,
{
    pub fn get_start_ix(&self) -> NodeIx {
        self.start_ix
    }

    pub fn get_weight(&self) -> &E {
        &self.weight
    }

    pub fn get_edges(&self, node_ix: NodeIx) -> &[(NodeIx, E)] {
        self.adj_list[node_ix].as_slice()
    }

    pub fn dfs(&self) -> MstDfsIter<'_, E>
    where
        E: Default,
    {
        MstDfsIter {
            mst: self,
            stack: self.adj_list[self.start_ix]
                .iter()
                .map(|(to_ix, weight)| (self.start_ix, *to_ix, *weight))
                .collect(),
        }
    }
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

pub fn prim<N>(graph: &impl Graph<N, NotNan<f64>>, start_ix: NodeIx) -> Mst<NotNan<f64>> {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct WeightedEdgeTarget(NotNan<f64>, NodeIx, NodeIx);

    let mut in_mst = vec![false; graph.size().0 as usize];
    let mut fringe = BinaryHeap::new();

    fn add_node_to_mst<NN>(
        ix: NodeIx,
        graph: &impl Graph<NN, NotNan<f64>>,
        in_mst: &mut Vec<bool>,
        fringe: &mut BinaryHeap<Reverse<WeightedEdgeTarget>>,
    ) {
        in_mst[ix] = true;
        for (weight, to_ix) in graph.get_edges(ix).map(|(o, w)| (*w, o)) {
            if !in_mst[to_ix] {
                fringe.push(Reverse(WeightedEdgeTarget(weight, ix, to_ix)));
            }
        }
    }

    add_node_to_mst(start_ix, graph, &mut in_mst, &mut fringe);

    let mut mst: Mst<NotNan<f64>> = Mst {
        start_ix: start_ix,
        adj_list: vec![Vec::new(); graph.size().0 as usize],
        weight: Default::default(), // Zero
    };

    while let Some(Reverse(WeightedEdgeTarget(weight, from_ix, to_ix))) = fringe.pop() {
        if !in_mst[to_ix] {
            add_node_to_mst(to_ix, graph, &mut in_mst, &mut fringe);

            mst.adj_list[from_ix].push((to_ix, weight));
            mst.weight += weight;
        }
    }

    mst
}
