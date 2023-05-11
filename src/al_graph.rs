use std::{collections::HashMap, ops::Index};

use crate::{Graph, NodeIx};

#[derive(Clone, Copy)]
pub struct EdgeIx(u32);

impl<T> Index<EdgeIx> for Vec<T> {
    type Output = T;

    fn index(&self, index: EdgeIx) -> &T {
        &self[index.0 as usize]
    }
}

pub struct ALGraph<N, E> {
    adj_lists: HashMap<NodeIx, HashMap<NodeIx, EdgeIx>>,
    nodes: Vec<N>,
    edges: Vec<E>,
}

impl<N, E> ALGraph<N, E>
where
    N: Default + Clone,
{
    pub fn new(size: u32) -> Self {
        ALGraph {
            adj_lists: Default::default(),
            nodes: vec![Default::default(); size as usize],
            edges: Default::default(),
        }
    }
}

impl<N, E> Graph<N, E> for ALGraph<N, E> {
    fn add_edge(&mut self, from: NodeIx, to: NodeIx, weight: E) {
        if from >= self.size() {
            panic!("from index out of bounds")
        }
        if to >= self.size() {
            panic!("to index out of bounds")
        }

        let edge_ix = EdgeIx(self.edges.len() as u32);
        self.edges.push(weight);

        self.adj_lists.entry(from).or_default().insert(to, edge_ix);
        self.adj_lists.entry(to).or_default().insert(from, edge_ix);
    }

    fn get_edges(&self, node_ix: NodeIx) -> Box<dyn Iterator<Item = (NodeIx, &E)> + '_> {
        Box::new(self.adj_lists.get(&node_ix).into_iter().flat_map(|al| {
            al.iter()
                .map(|(node_ix, edge_ix)| (*node_ix, &self.edges[*edge_ix]))
        }))
    }

    fn get_all_edges(&self) -> Box<dyn Iterator<Item = (NodeIx, NodeIx, &E)> + '_> {
        Box::new(self.adj_lists.iter().flat_map(move |(from_ix, edges)| {
            edges.iter().filter_map(move |(to_ix, edge_ix)| {
                if from_ix <= to_ix {
                    Some((*from_ix, *to_ix, &self.edges[*edge_ix]))
                } else {
                    None
                }
            })
        }))
    }

    fn get_node(&self, node_ix: NodeIx) -> Option<&N> {
        self.nodes.get(node_ix.0 as usize)
    }

    fn get_node_mut(&mut self, node_ix: NodeIx) -> Option<&mut N> {
        self.nodes.get_mut(node_ix.0 as usize)
    }

    fn size(&self) -> NodeIx {
        NodeIx(self.nodes.len() as u32)
    }
}
