use std::ops::{Index, IndexMut};

pub mod al_graph;
pub mod algo;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct NodeIx(pub u32);

impl From<NodeIx> for usize {
    fn from(value: NodeIx) -> Self {
        value.0 as usize
    }
}

impl<T> Index<NodeIx> for Vec<T> {
    type Output = T;

    fn index(&self, index: NodeIx) -> &T {
        &self[index.0 as usize]
    }
}

impl<T> IndexMut<NodeIx> for Vec<T> {
    fn index_mut(&mut self, index: NodeIx) -> &mut T {
        &mut self[index.0 as usize]
    }
}

pub trait Graph<N, E> {
    fn add_edge(&mut self, from: NodeIx, to: NodeIx, weight: E);
    fn get_edges(&self, node_ix: NodeIx) -> Box<dyn Iterator<Item = (NodeIx, &E)> + '_>;
    fn get_edge_weight(&self, from_ix: NodeIx, to_ix: NodeIx) -> Option<&E>;
    fn get_all_edges(&self) -> Box<dyn Iterator<Item = (NodeIx, NodeIx, &E)> + '_>;
    fn get_node(&self, node_ix: NodeIx) -> Option<&N>;
    fn get_node_mut(&mut self, node_ix: NodeIx) -> Option<&mut N>;
    fn size(&self) -> NodeIx;
}
