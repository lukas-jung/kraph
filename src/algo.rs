use std::{
    collections::{HashMap, HashSet},
    ops::DerefMut,
};

use ordered_float::NotNan;

use crate::Graph;

pub fn count_components<N, E>(graph: impl Graph<N, E>) -> u32 {
    let size = graph.size();
    let mut visited = vec![false; size as usize];
    let mut count = 0;

    for ix in 0..size {
        if !std::mem::replace(&mut visited[ix as usize], true) {
            let mut stack = vec![ix];

            while let Some(current) = stack.pop() {
                for &(adj_ix, _) in graph.get_edges(current) {
                    if !std::mem::replace(&mut visited[adj_ix as usize], true) {
                        stack.push(adj_ix);
                    }
                }
            }

            count += 1;
        }
    }
    count
}

pub fn kruskal<N>(graph: impl Graph<N, NotNan<f64>>) -> NotNan<f64> {
    let mut edges: Vec<_> = graph.get_all_edges().deref_mut().collect();
    edges.sort_by_key(|e| e.2);

    let mut id2set: HashMap<u32, HashSet<u32>> =
        (0..graph.size()).map(|i| (i, [i].into())).collect();
    let mut ix2id: Vec<u32> = (0..graph.size()).collect();

    fn union(
        target_set_id: u32,
        other_set_id: u32,
        id2set: &mut HashMap<u32, HashSet<u32>>,
        ix2id: &mut Vec<u32>,
    ) {
        for &ix in &id2set[&other_set_id] {
            ix2id[ix as usize] = target_set_id;
        }
        let other_set = id2set.remove(&other_set_id).unwrap();
        id2set.get_mut(&target_set_id).unwrap().extend(other_set);
    }

    let mut result = NotNan::new(0f64).unwrap();
    for edge in edges.iter().copied() {
        let from_id = ix2id[edge.0 as usize];
        let to_id = ix2id[edge.1 as usize];

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
